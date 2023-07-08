use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    bytecode::*,
    bytes_util::*,
    error::CodegenError,
    evm::Opcode,
    evm_version::EVMVersion,
    prelude::{MacroArg::Ident, Span, TokenKind},
};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// A contained literal
pub type Literal = [u8; 32];

/// A File Path
///
/// Used for parsing the huff imports.
pub type FilePath = PathBuf;

/// An AST-level Span
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstSpan(pub Vec<Span>);

impl AstSpan {
    /// Coalesce Multiple Spans Into an error string
    pub fn error(&self, hint: Option<&String>) -> String {
        let file_to_source_map =
            self.0.iter().fold(BTreeMap::<String, Vec<&Span>>::new(), |mut m, s| {
                let file_name =
                    s.file.as_ref().map(|f2| f2.path.clone()).unwrap_or_else(|| "".to_string());
                let mut new_vec: Vec<&Span> = m.get(&file_name).cloned().unwrap_or_default();
                new_vec.push(s);
                m.insert(file_name, new_vec);
                m
            });
        let source_str = file_to_source_map.iter().filter(|fs| !fs.0.is_empty()).fold(
            "".to_string(),
            |s, fs| {
                let start = fs.1.iter().map(|fs2| fs2.start).min().unwrap_or(0);
                let end = fs.1.iter().map(|fs2| fs2.end).max().unwrap_or(0);
                let newline_s = if s.is_empty() { "".to_string() } else { format!("{s}\n") };
                if start.eq(&0) && end.eq(&0) {
                    format!("{newline_s}-> {}:{start}\n   > 0|", fs.0)
                } else {
                    format!(
                        "{}-> {}:{}-{}{}",
                        newline_s,
                        fs.0,
                        start,
                        end,
                        fs.1.iter()
                            .map(|sp| sp.source_seg())
                            .filter(|ss| !ss.is_empty())
                            .collect::<Vec<String>>()
                            .into_iter()
                            .unique()
                            .fold("".to_string(), |acc, ss| { format!("{acc}{ss}") })
                    )
                }
            },
        );
        // Add in optional hint message
        format!("{}{source_str}", hint.map(|msg| format!("{msg}\n")).unwrap_or_default())
    }

    /// Print just the file for missing
    pub fn file(&self) -> String {
        self.0.iter().fold("".to_string(), |acc, span| match &span.file {
            Some(fs) => format!("-> {}\n{acc}", fs.path),
            None => Default::default(),
        })
    }
}

/// A Huff Contract Representation
///
/// This is the representation of a contract as it is parsed from huff source code.
/// Thus, it is also the root of the AST.
///
/// For examples of Huff contracts, see the [huff-examples repository](https://github.com/huff-language/huff-examples).
#[derive(Debug, Default, Clone)]
pub struct Contract {
    /// Macro definitions
    pub macros: Vec<MacroDefinition>,
    /// Invocations of macros
    pub invocations: Vec<MacroInvocation>,
    /// File Imports
    pub imports: Vec<FilePath>,
    /// Constants
    pub constants: Arc<Mutex<Vec<ConstantDefinition>>>,
    /// Custom Errors
    pub errors: Vec<ErrorDefinition>,
    /// Functions
    pub functions: Vec<FunctionDefinition>,
    /// Events
    pub events: Vec<EventDefinition>,
    /// Tables
    pub tables: Vec<TableDefinition>,
}

impl Contract {
    /// Returns the first macro that matches the provided name
    pub fn find_macro_by_name(&self, name: &str) -> Option<&MacroDefinition> {
        if let Some(m) = self.macros.iter().find(|m| m.name == name) {
            Some(m)
        } else {
            tracing::warn!("Failed to find macro \"{}\" in contract", name);
            None
        }
    }

    /// Returns the first table that matches the provided name
    pub fn find_table_by_name(&self, name: &str) -> Option<TableDefinition> {
        if let Some(t) = self.tables.iter().find(|t| t.name == name) {
            Some(t.clone())
        } else {
            tracing::warn!("Failed to find table \"{}\" in contract", name);
            None
        }
    }

    /// Derives the FreeStoragePointers into their bytes32 representation
    pub fn derive_storage_pointers(&mut self) {
        let mut storage_pointers: Vec<(String, [u8; 32])> = Vec::new();
        let mut last_assigned_free_pointer = 0;

        // Derive Constructor Storage Pointers
        match self.find_macro_by_name("CONSTRUCTOR") {
            Some(m) => self.recurse_ast_constants(
                m,
                &mut storage_pointers,
                &mut last_assigned_free_pointer,
                false,
            ),
            None => {
                // The constructor is not required, so we can just warn
                tracing::warn!(target: "ast", "'CONSTRUCTOR' MACRO NOT FOUND WHILE DERIVING STORAGE POINTERS!")
            }
        }

        // Derive Main Storage Pointers
        match self.find_macro_by_name("MAIN") {
            Some(m) => self.recurse_ast_constants(
                m,
                &mut storage_pointers,
                &mut last_assigned_free_pointer,
                false,
            ),
            None => {
                tracing::error!(target: "ast", "'MAIN' MACRO NOT FOUND WHILE DERIVING STORAGE POINTERS!")
            }
        }

        tracing::debug!(target: "ast", "Generate Storage pointers: {:?}", storage_pointers);
        tracing::debug!(target: "ast", "ALL AST CONSTANTS: {:?}", storage_pointers);

        // Set all the constants to their new values
        for c in self.constants.lock().unwrap().iter_mut() {
            match storage_pointers
                .iter()
                .filter(|pointer| pointer.0.eq(&c.name))
                .collect::<Vec<&(String, [u8; 32])>>()
                .get(0)
            {
                Some(p) => {
                    *c = ConstantDefinition {
                        name: c.name.to_string(),
                        value: ConstVal::Literal(p.1),
                        span: c.span.clone(),
                    };
                }
                None => {
                    tracing::warn!(target: "ast", "SET STORAGE POINTER BUT FAILED TO SET DERIVED CONSTANT VALUE FOR \"{}\"", c.name)
                }
            }
        }
    }

    /// Recurse down an AST Macro Definition to set Storage Pointers
    ///
    /// ## Overview
    ///
    /// For each statement in the macro definition:
    ///     - If it's a free storage pointer constant, set the constant value if not already set and
    ///       updated out `last_p` tracker value
    ///     - If it's a literal constant, we can set the constant value directly to the literal if
    ///       not already set
    ///     - If it's a macro invocation, look for the macro definition and recurse into that macro
    ///       definition using `recurse_ast_constants`
    pub fn recurse_ast_constants(
        &self,
        macro_def: &MacroDefinition,
        storage_pointers: &mut Vec<(String, [u8; 32])>,
        last_p: &mut i32,
        checking_constructor: bool,
    ) {
        let mut statements = macro_def.statements.clone();

        let mut i = 0;
        loop {
            if i >= statements.len() {
                break
            }
            match &statements[i].clone().ty {
                StatementType::Constant(const_name) => {
                    self.assign_free_storage_pointers(
                        const_name,
                        &macro_def.name,
                        storage_pointers,
                        last_p,
                    );
                }
                StatementType::MacroInvocation(mi) => {
                    tracing::debug!(target: "ast", "Found macro invocation: \"{}\" in macro def: \"{}\"!", mi.macro_name, macro_def.name);

                    // Check for constant references in macro arguments
                    let mut constant_args: Vec<String> = Vec::new();
                    for arg in &mi.args {
                        // check if it is a constant
                        if let Ident(name) = arg {
                            self.constants.lock().unwrap().iter().for_each(|constant| {
                                if name == &constant.name {
                                    tracing::debug!(target: "ast", "CONSTANT FOUND AS MACRO PARAMETER {}", name);
                                    constant_args.push(name.to_string());
                                }
                            })
                        }
                    }
                    // Assign constants that reference the Free Storage Pointer
                    for constant_arg in constant_args {
                        self.assign_free_storage_pointers(
                            &constant_arg,
                            &macro_def.name,
                            storage_pointers,
                            last_p,
                        );
                    }

                    match self
                        .macros
                        .iter()
                        .filter(|md| md.name.eq(&mi.macro_name))
                        .collect::<Vec<&MacroDefinition>>()
                        .get(0)
                    {
                        Some(&md) => {
                            if md.name.eq("CONSTRUCTOR") {
                                if !checking_constructor {
                                    self.recurse_ast_constants(md, storage_pointers, last_p, true);
                                }
                            } else {
                                self.recurse_ast_constants(
                                    md,
                                    storage_pointers,
                                    last_p,
                                    checking_constructor,
                                );
                            }
                        }
                        None => {
                            tracing::warn!(target: "ast", "MACRO \"{}\" INVOKED BUT NOT FOUND IN AST!", mi.macro_name)
                        }
                    }
                }
                StatementType::BuiltinFunctionCall(bfc) => {
                    tracing::debug!(target: "ast", "Deriving Storage Pointers: Found builtin function {:?}", bfc.kind);
                    for a in &bfc.args {
                        if let Some(name) = &a.name {
                            match self
                                .macros
                                .iter()
                                .filter(|md| md.name.eq(name))
                                .collect::<Vec<&MacroDefinition>>()
                                .get(0)
                            {
                                Some(&md) => {
                                    if md.name.eq("CONSTRUCTOR") {
                                        if !checking_constructor {
                                            self.recurse_ast_constants(
                                                md,
                                                storage_pointers,
                                                last_p,
                                                true,
                                            );
                                        }
                                    } else {
                                        self.recurse_ast_constants(
                                            md,
                                            storage_pointers,
                                            last_p,
                                            checking_constructor,
                                        );
                                    }
                                }
                                None => {
                                    tracing::warn!(target: "ast", "BUILTIN HAS ARG NAME \"{}\" BUT NOT FOUND IN AST!", name)
                                }
                            }
                        }
                    }
                }
                StatementType::Label(l) => {
                    for state in l.inner.iter().rev() {
                        statements.insert(i + 1, state.clone());
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // Breadth-first
        // if !macros_to_recurse.is_empty() {
        //     let next_md = macros_to_recurse.remove(0);
        //     self.recurse_ast_constants(next_md, storage_pointers, last_p, macros_to_recurse);
        // }
    }

    fn assign_free_storage_pointers(
        &self,
        const_name: &String,
        macro_name: &String,
        storage_pointers: &mut Vec<(String, [u8; 32])>,
        last_p: &mut i32,
    ) {
        tracing::debug!(target: "ast", "Found constant \"{}\" in macro def \"{}\" statements!", const_name, macro_name);
        if storage_pointers
            .iter()
            .filter(|pointer| pointer.0.eq(const_name))
            .collect::<Vec<&(String, [u8; 32])>>()
            .get(0)
            .is_none()
        {
            tracing::debug!(target: "ast", "No storage pointer already set for \"{}\"!", const_name);
            // Get the associated constant
            match self
                .constants
                .lock()
                .unwrap()
                .iter()
                .filter(|c| c.name.eq(const_name))
                .collect::<Vec<&ConstantDefinition>>()
                .get(0)
            {
                Some(c) => {
                    let new_value = match c.value {
                        ConstVal::Literal(l) => l,
                        ConstVal::FreeStoragePointer(_) => {
                            let old_p = *last_p;
                            *last_p += 1;
                            str_to_bytes32(&format!("{old_p}"))
                        }
                    };
                    storage_pointers.push((const_name.to_string(), new_value));
                }
                None => {
                    tracing::warn!(target: "ast", "CONSTANT \"{}\" NOT FOUND IN AST CONSTANTS", const_name)
                }
            }
        }
    }

    /// Add override constants to the AST
    ///
    /// ## Overview
    ///
    /// For each override constant, add it to the AST if it doesn't already exist. Override
    /// constants can be passed in via the CLI.
    pub fn add_override_constants(&self, override_constants: &Option<BTreeMap<&str, Literal>>) {
        if let Some(override_constants) = override_constants {
            for (name, value) in override_constants {
                let mut constants = self.constants.lock().unwrap();
                if let Some(c) = constants.iter_mut().find(|c| c.name.as_str().eq(*name)) {
                    c.value = ConstVal::Literal(*value);
                } else {
                    constants.push(ConstantDefinition {
                        name: name.to_string(),
                        value: ConstVal::Literal(*value),
                        span: AstSpan::default(),
                    });
                }
            }
        }
    }
}

/// An argument's location
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArgumentLocation {
    /// Memory location
    #[default]
    Memory,
    /// Storage location
    Storage,
    /// Calldata location
    Calldata,
}

/// A function, event, or macro argument
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Argument {
    /// Type of the argument
    pub arg_type: Option<String>,
    /// Optional Argument Location
    pub arg_location: Option<ArgumentLocation>,
    /// The name of the argument
    pub name: Option<String>,
    /// Is the argument indexed? TODO: should be valid for event arguments ONLY
    pub indexed: bool,
    /// The argument span
    pub span: AstSpan,
}

/// A Function Signature
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionDefinition {
    /// The name of the function
    pub name: String,
    /// The function signature
    pub signature: [u8; 4],
    /// The parameters of the function
    pub inputs: Vec<Argument>,
    /// The function type
    pub fn_type: FunctionType,
    /// The return values of the function
    pub outputs: Vec<Argument>,
    /// The span of the function
    pub span: AstSpan,
}

/// Function Types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionType {
    /// Viewable Function
    View,
    /// Payable Function
    Payable,
    /// Non Payable Function
    NonPayable,
    /// Pure Function
    Pure,
}

impl FunctionType {
    /// Get the string representation of the function type for usage in Solidity interface
    /// generation.
    pub fn interface_mutability(&self) -> &str {
        match self {
            FunctionType::View => " view",
            FunctionType::Pure => " pure",
            _ => "", // payable / nonpayable types not valid in Solidity interfaces
        }
    }
}

/// An Event Signature
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventDefinition {
    /// The name of the event
    pub name: String,
    /// The parameters of the event
    pub parameters: Vec<Argument>,
    /// The event span
    pub span: AstSpan,
    /// The event hash
    pub hash: Literal,
}

/// A Table Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableDefinition {
    /// The name of the table
    pub name: String,
    /// The table kind
    pub kind: TableKind,
    /// The table's statements
    pub statements: Vec<Statement>,
    /// Size of table
    pub size: Literal,
    /// The table span
    pub span: AstSpan,
}

impl TableDefinition {
    /// Public associated function that instantiates a TableDefinition from a string
    pub fn new(
        name: String,
        kind: TableKind,
        statements: Vec<Statement>,
        size: Literal,
        span: AstSpan,
    ) -> Self {
        TableDefinition { name, kind, statements, size, span }
    }
}

/// A Table Kind
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableKind {
    /// A regular jump table
    JumpTable,
    /// A packed jump table
    JumpTablePacked,
    /// A code table
    CodeTable,
}

impl From<TokenKind> for TableKind {
    /// Public associated function that converts a TokenKind to a TableKind
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::JumpTable => TableKind::JumpTable,
            TokenKind::JumpTablePacked => TableKind::JumpTablePacked,
            TokenKind::CodeTable => TableKind::CodeTable,
            _ => panic!("Invalid Token Kind"), // TODO: Better error handling
        }
    }
}

/// A Macro Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacroDefinition {
    /// The Macro Name
    pub name: String,
    /// The macro's decorator
    pub decorator: Option<Decorator>,
    /// A list of Macro parameters
    pub parameters: Vec<Argument>,
    /// A list of Statements contained in the Macro
    pub statements: Vec<Statement>,
    /// The take size
    pub takes: usize,
    /// The return size
    pub returns: usize,
    /// The Span of the Macro Definition
    pub span: AstSpan,
    /// Is the macro a function (outlined)?
    pub outlined: bool,
    /// Is the macro a test?
    pub test: bool,
}

impl ToIRBytecode<CodegenError> for MacroDefinition {
    fn to_irbytecode(&self, evm_version: &EVMVersion) -> Result<IRBytecode, CodegenError> {
        let inner_irbytes: Vec<IRBytes> =
            MacroDefinition::to_irbytes(evm_version, &self.statements);
        Ok(IRBytecode(inner_irbytes))
    }
}

impl MacroDefinition {
    /// Public associated function that instantiates a MacroDefinition.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        decorator: Option<Decorator>,
        parameters: Vec<Argument>,
        statements: Vec<Statement>,
        takes: usize,
        returns: usize,
        spans: Vec<Span>,
        outlined: bool,
        test: bool,
    ) -> Self {
        MacroDefinition {
            name,
            decorator,
            parameters,
            statements,
            takes,
            returns,
            span: AstSpan(spans),
            outlined,
            test,
        }
    }

    /// Translate statements into IRBytes
    pub fn to_irbytes<'a>(
        evm_version: &EVMVersion,
        statements: &'a [Statement],
    ) -> Vec<IRBytes<'a>> {
        let mut inner_irbytes: Vec<IRBytes> = vec![];

        let mut statement_iter = statements.iter();
        while let Some(statement) = statement_iter.next() {
            match &statement.ty {
                StatementType::Literal(l) => {
                    let push_bytes = literal_gen(evm_version, l);
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Bytes(Bytes(push_bytes)),
                        span: &statement.span,
                    });
                }
                StatementType::Opcode(o) => {
                    let opcode_str = o.string();
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Bytes(Bytes(opcode_str)),
                        span: &statement.span,
                    });
                    // If the opcode is a push that takes a literal value, we need to consume the
                    // next statement, which must be a literal as checked in the parser
                    if o.is_value_push() {
                        match statement_iter.next() {
                            Some(Statement { ty: StatementType::Literal(l), span: _ }) => {
                                let hex_literal: String = bytes32_to_string(l, false);
                                let prefixed_hex_literal = o.prefix_push_literal(&hex_literal);
                                inner_irbytes.push(IRBytes {
                                    ty: IRByteType::Bytes(Bytes(prefixed_hex_literal)),
                                    span: &statement.span,
                                });
                            }
                            _ => {
                                // We have a push without a literal - this should be caught by the
                                // parser
                                panic!("Invalid push statement");
                            }
                        }
                    }
                }
                StatementType::Code(c) => {
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Bytes(Bytes(c.to_owned())),
                        span: &statement.span,
                    });
                }
                StatementType::MacroInvocation(mi) => {
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::MacroInvocation(mi.clone()),
                            span: statement.span.clone(),
                        }),
                        span: &statement.span,
                    });
                }
                StatementType::Constant(name) => {
                    // Constant needs to be evaluated at the top-level
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Constant(name.to_owned()),
                        span: &statement.span,
                    });
                }
                StatementType::ArgCall(arg_name) => {
                    // Arg call needs to use a destination defined in the calling macro context
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::ArgCall(arg_name.to_owned()),
                        span: &statement.span,
                    });
                }
                StatementType::LabelCall(jump_to) => {
                    /* Jump To doesn't translate directly to bytecode */
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::LabelCall(jump_to.to_string()),
                            span: statement.span.clone(),
                        }),
                        span: &statement.span,
                    });
                }
                StatementType::Label(l) => {
                    /* Jump Dests don't translate directly to bytecode */
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::Label(l.clone()),
                            span: statement.span.clone(),
                        }),
                        span: &statement.span,
                    });

                    // Recurse label statements to IRBytes Bytes
                    inner_irbytes.append(&mut MacroDefinition::to_irbytes(evm_version, &l.inner));
                }
                StatementType::BuiltinFunctionCall(builtin) => {
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::BuiltinFunctionCall(builtin.clone()),
                            span: statement.span.clone(),
                        }),
                        span: &statement.span,
                    });
                }
            }
        }

        inner_irbytes
    }
}

/// A Macro Invocation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacroInvocation {
    /// The Macro Name
    pub macro_name: String,
    /// A list of Macro arguments
    pub args: Vec<MacroArg>,
    /// The Macro Invocation Span
    pub span: AstSpan,
}

/// An argument passed when invoking a maco
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MacroArg {
    /// Macro Literal Argument
    Literal(Literal),
    /// Macro Iden String Argument
    Ident(String),
    /// An Arg Call
    ArgCall(String),
}

/// Free Storage Pointer Unit Struct
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreeStoragePointer;

/// A Constant Value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstVal {
    /// A literal value for the constant
    Literal(Literal),
    /// A Free Storage Pointer
    FreeStoragePointer(FreeStoragePointer),
}

/// A Constant Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstantDefinition {
    /// The Constant name
    pub name: String,
    /// The Constant value
    pub value: ConstVal,
    /// The Span of the Constant Definition
    pub span: AstSpan,
}

/// An Error Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorDefinition {
    /// The Error name
    pub name: String,
    /// The Error's selector
    pub selector: [u8; 4],
    /// The parameters of the error
    pub parameters: Vec<Argument>,
    /// The Span of the Constant Definition
    pub span: AstSpan,
}

/// A Jump Destination
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label {
    /// The JumpDest Name
    pub name: String,
    /// Statements Inside The JumpDest
    pub inner: Vec<Statement>,
    /// The label span
    pub span: AstSpan,
}

/// A Builtin Function Call
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BuiltinFunctionCall {
    /// The Builtin Kind
    pub kind: BuiltinFunctionKind,
    /// Arguments for the builtin function call.
    /// TODO: Maybe make a better type for this other than `Argument`? Would be nice if it pointed
    ///       directly to the macro/table.
    pub args: Vec<Argument>,
    /// The builtin function call span
    pub span: AstSpan,
}

/// A Builtin Function Kind
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BuiltinFunctionKind {
    /// Table size function
    Tablesize,
    /// Code size function
    Codesize,
    /// Table start function
    Tablestart,
    /// Function signature function
    FunctionSignature,
    /// Event hash function
    EventHash,
    /// Error selector function
    Error,
    /// Rightpad function
    RightPad,
    /// Dynamic constructor arg function
    DynConstructorArg,
    /// Inject Raw Bytes
    Verbatim,
}

impl From<String> for BuiltinFunctionKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "__tablesize" => BuiltinFunctionKind::Tablesize,
            "__codesize" => BuiltinFunctionKind::Codesize,
            "__tablestart" => BuiltinFunctionKind::Tablestart,
            "__FUNC_SIG" => BuiltinFunctionKind::FunctionSignature,
            "__EVENT_HASH" => BuiltinFunctionKind::EventHash,
            "__ERROR" => BuiltinFunctionKind::Error,
            "__RIGHTPAD" => BuiltinFunctionKind::RightPad,
            "__CODECOPY_DYN_ARG" => BuiltinFunctionKind::DynConstructorArg,
            "__VERBATIM" => BuiltinFunctionKind::Verbatim,
            _ => panic!("Invalid Builtin Function Kind"), /* This should never be reached,
                                                           * builtins are validated with a
                                                           * `try_from` call in the lexer. */
        }
    }
}

impl TryFrom<&String> for BuiltinFunctionKind {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, <BuiltinFunctionKind as TryFrom<&String>>::Error> {
        match value.as_str() {
            "__tablesize" => Ok(BuiltinFunctionKind::Tablesize),
            "__codesize" => Ok(BuiltinFunctionKind::Codesize),
            "__tablestart" => Ok(BuiltinFunctionKind::Tablestart),
            "__FUNC_SIG" => Ok(BuiltinFunctionKind::FunctionSignature),
            "__EVENT_HASH" => Ok(BuiltinFunctionKind::EventHash),
            "__ERROR" => Ok(BuiltinFunctionKind::Error),
            "__RIGHTPAD" => Ok(BuiltinFunctionKind::RightPad),
            "__CODECOPY_DYN_ARG" => Ok(BuiltinFunctionKind::DynConstructorArg),
            "__VERBATIM" => Ok(BuiltinFunctionKind::Verbatim),
            _ => Err(()),
        }
    }
}

/// A Statement
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Statement {
    /// The type of statement
    pub ty: StatementType,
    /// The span of the Statement
    pub span: AstSpan,
}

/// The Statement Type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatementType {
    /// A Literal Statement
    Literal(Literal),
    /// An Opcode Statement
    Opcode(Opcode),
    /// A Code Statement
    Code(String),
    /// A Macro Invocation Statement
    MacroInvocation(MacroInvocation),
    /// A Constant Push
    Constant(String),
    /// An Arg Call
    ArgCall(String),
    /// A Label
    Label(Label),
    /// A Label Reference/Call
    LabelCall(String),
    /// A built-in function call
    BuiltinFunctionCall(BuiltinFunctionCall),
}

impl Display for StatementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementType::Literal(l) => write!(f, "LITERAL: {}", bytes32_to_string(l, true)),
            StatementType::Opcode(o) => write!(f, "OPCODE: {o}"),
            StatementType::Code(s) => write!(f, "CODE: {s}"),
            StatementType::MacroInvocation(m) => {
                write!(f, "MACRO INVOCATION: {}", m.macro_name)
            }
            StatementType::Constant(c) => write!(f, "CONSTANT: {c}"),
            StatementType::ArgCall(c) => write!(f, "ARG CALL: {c}"),
            StatementType::Label(l) => write!(f, "LABEL: {}", l.name),
            StatementType::LabelCall(l) => write!(f, "LABEL CALL: {l}"),
            StatementType::BuiltinFunctionCall(b) => {
                write!(f, "BUILTIN FUNCTION CALL: {:?}", b.kind)
            }
        }
    }
}

/// A decorator tag
///
/// At the moment, the decorator tag can only be placed over test definitions. Developers
/// can use decorators to define environment variables and other metadata for their individual
/// tests.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decorator {
    /// Vector of flags passed within the decorator
    pub flags: Vec<DecoratorFlag>,
}

/// A decorator flag
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecoratorFlag {
    /// Sets the calldata of the test call transaction
    Calldata(String),
    /// Sets the value of the test call transaction
    Value(Literal),
}

impl TryFrom<&String> for DecoratorFlag {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "calldata" => Ok(DecoratorFlag::Calldata(String::default())),
            "value" => Ok(DecoratorFlag::Value(Literal::default())),
            _ => Err(()),
        }
    }
}
