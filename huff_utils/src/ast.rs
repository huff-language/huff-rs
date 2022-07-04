use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    bytecode::*,
    bytes_util::*,
    error::CodegenError,
    evm::Opcode,
    prelude::{Span, TokenKind},
};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    path::PathBuf,
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
                let newline_s = if s.is_empty() { "".to_string() } else { format!("{}\n", s) };
                if start.eq(&0) && end.eq(&0) {
                    format!("{}-> {}:{}\n   > 0|", newline_s, fs.0, start)
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
                            .fold("".to_string(), |acc, ss| { format!("{}{}", acc, ss) })
                    )
                }
            },
        );
        // Add in optional hint message
        format!(
            "{}{}",
            hint.map(|msg| format!("{}\n", /* " ".repeat(7), */ msg)).unwrap_or_default(),
            source_str
        )
    }

    /// Print just the file for missing
    pub fn file(&self) -> String {
        self.0.iter().fold("".to_string(), |acc, span| match &span.file {
            Some(fs) => format!("-> {}\n{}", fs.path, acc),
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Contract {
    /// Macro definitions
    pub macros: Vec<MacroDefinition>,
    /// Invocations of macros
    pub invocations: Vec<MacroInvocation>,
    /// File Imports
    pub imports: Vec<FilePath>,
    /// Constants
    pub constants: Vec<ConstantDefinition>,
    /// Immutables
    pub immutables: Vec<ImmutableDefinition>,
    /// Functions
    pub functions: Vec<Function>,
    /// Events
    pub events: Vec<Event>,
    /// Tables
    pub tables: Vec<TableDefinition>,
}

impl Contract {
    /// Returns the first macro that matches the provided name
    pub fn find_macro_by_name(&self, name: &str) -> Option<MacroDefinition> {
        if let Some(m) = self.macros.iter().find(|m| m.name == name) {
            Some(m.clone())
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
                &m,
                &mut storage_pointers,
                &mut last_assigned_free_pointer,
            ),
            None => {
                tracing::error!(target: "ast", "'CONSTRUCTOR' MACRO NOT FOUND WHILE DERIVING STORAGE POINTERS!")
            }
        }

        // Derive Main Storage Pointers
        match self.find_macro_by_name("MAIN") {
            Some(m) => self.recurse_ast_constants(
                &m,
                &mut storage_pointers,
                &mut last_assigned_free_pointer,
            ),
            None => {
                tracing::error!(target: "ast", "'MAIN' MACRO NOT FOUND WHILE DERIVING STORAGE POINTERS!")
            }
        }

        tracing::debug!(target: "ast", "Generate Storage pointers: {:?}", storage_pointers);
        tracing::debug!(target: "ast", "ALL AST CONSTANTS: {:?}", storage_pointers);

        // Set all the constants to their new values
        for c in &mut self.constants {
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
    ) {
        let mut statements = macro_def.statements.clone();
        let mut i = 0;
        loop {
            if i >= statements.len() {
                break
            }
            match &statements[i].clone().ty {
                StatementType::Constant(const_name) => {
                    tracing::debug!(target: "ast", "Found constant \"{}\" in macro def \"{}\" statements!", const_name, macro_def.name);
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
                                        str_to_bytes32(&format!("{}", old_p))
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
                StatementType::MacroInvocation(mi) => {
                    tracing::debug!(target: "ast", "Found macro invocation: \"{}\" in macro def: \"{}\"!", mi.macro_name, macro_def.name);
                    match self
                        .macros
                        .iter()
                        .filter(|md| md.name.eq(&mi.macro_name))
                        .collect::<Vec<&MacroDefinition>>()
                        .get(0)
                    {
                        Some(&md) => self.recurse_ast_constants(md, storage_pointers, last_p),
                        None => {
                            tracing::warn!(target: "ast", "MACRO \"{}\" INVOKED BUT NOT FOUND IN AST!", mi.macro_name)
                        }
                    }
                }
                StatementType::BuiltinFunctionCall(bfc) => {
                    tracing::debug!(target: "ast", "Deriving Storage Pointrs: Found builtin function {:?}", bfc.kind);
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
                                    self.recurse_ast_constants(md, storage_pointers, last_p)
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
}

/// A function, event, or macro argument
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Argument {
    /// Type of the argument
    pub arg_type: Option<String>,
    /// The name of the argument
    pub name: Option<String>,
    /// Is the argument indexed? TODO: should be valid for event arguments ONLY
    pub indexed: bool,
    /// The argument span
    pub span: AstSpan,
}

/// A Function Signature
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function {
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
pub struct Event {
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
}

impl ToIRBytecode<CodegenError> for MacroDefinition {
    fn to_irbytecode(&self) -> Result<IRBytecode, CodegenError> {
        let inner_irbytes: Vec<IRBytes> = MacroDefinition::to_irbytes(&self.statements);
        Ok(IRBytecode(inner_irbytes))
    }
}

impl MacroDefinition {
    /// Public associated function that instantiates a MacroDefinition.
    pub fn new(
        name: String,
        parameters: Vec<Argument>,
        statements: Vec<Statement>,
        takes: usize,
        returns: usize,
        spans: Vec<Span>,
    ) -> Self {
        MacroDefinition { name, parameters, statements, takes, returns, span: AstSpan(spans) }
    }

    /// Translate statements into IRBytes
    pub fn to_irbytes(statements: &[Statement]) -> Vec<IRBytes> {
        let mut inner_irbytes: Vec<IRBytes> = vec![];

        statements.iter().for_each(|statement| {
            match &statement.ty {
                StatementType::Literal(l) => {
                    let hex_literal: String = bytes32_to_string(l, false);
                    let push_bytes = format!("{:02x}{}", 95 + hex_literal.len() / 2, hex_literal);
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Bytes(Bytes(push_bytes)),
                        span: statement.span.clone(),
                    });
                }
                StatementType::Opcode(o) => {
                    let opcode_str = o.string();
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Bytes(Bytes(opcode_str)),
                        span: statement.span.clone(),
                    });
                }
                StatementType::MacroInvocation(mi) => {
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::MacroInvocation(mi.clone()),
                            span: statement.span.clone(),
                        }),
                        span: statement.span.clone(),
                    });
                }
                StatementType::Constant(name) => {
                    // Constant needs to be evaluated at the top-level
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Constant(name.to_string()),
                        span: statement.span.clone(),
                    });
                }
                StatementType::Immutable(name) => {
                    // Constant needs to be evaluated at the top-level
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Immutable(name.to_string()),
                        span: statement.span.clone(),
                    });
                }
                StatementType::ArgCall(arg_name) => {
                    // Arg call needs to use a destination defined in the calling macro context
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::ArgCall(arg_name.to_string()),
                        span: statement.span.clone(),
                    });
                }
                StatementType::LabelCall(jump_to) => {
                    /* Jump To doesn't translate directly to bytecode ? */
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::LabelCall(jump_to.to_string()),
                            span: statement.span.clone(),
                        }),
                        span: statement.span.clone(),
                    });
                }
                StatementType::Label(l) => {
                    /* Jump Dests don't translate directly to bytecode ? */
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::Label(l.clone()),
                            span: statement.span.clone(),
                        }),
                        span: statement.span.clone(),
                    });

                    // Recurse label statements to IRBytes Bytes
                    inner_irbytes.append(&mut MacroDefinition::to_irbytes(&l.inner));
                }
                StatementType::BuiltinFunctionCall(builtin) => {
                    inner_irbytes.push(IRBytes {
                        ty: IRByteType::Statement(Statement {
                            ty: StatementType::BuiltinFunctionCall(builtin.clone()),
                            span: statement.span.clone(),
                        }),
                        span: statement.span.clone(),
                    });
                }
            }
        });

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

/// An Immutable Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImmutableDefinition {
    /// The name
    pub name: String,
    /// The value - can only be set once
    pub value: Option<ConstVal>,
    /// The Span of the def
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
    /// directly to the macro/table.
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
}

impl From<&str> for BuiltinFunctionKind {
    fn from(s: &str) -> Self {
        match s {
            "__tablesize" => BuiltinFunctionKind::Tablesize,
            "__codesize" => BuiltinFunctionKind::Codesize,
            "__tablestart" => BuiltinFunctionKind::Tablestart,
            "__FUNC_SIG" => BuiltinFunctionKind::FunctionSignature,
            "__EVENT_HASH" => BuiltinFunctionKind::EventHash,
            _ => panic!("Invalid Builtin Function Kind"), // TODO: Better error handling
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
    /// A Macro Invocation Statement
    MacroInvocation(MacroInvocation),
    /// A Constant Push
    Constant(String),
    /// An Immutable
    Immutable(String),
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
            StatementType::Opcode(o) => write!(f, "OPCODE: {}", o),
            StatementType::MacroInvocation(m) => {
                write!(f, "MACRO INVOCATION: {}", m.macro_name)
            }
            StatementType::Constant(c) => write!(f, "CONSTANT: {}", c),
            StatementType::Immutable(c) => write!(f, "IMMUTABLE: {}", c),
            StatementType::ArgCall(c) => write!(f, "ARG CALL: {}", c),
            StatementType::Label(l) => write!(f, "LABEL: {}", l.name),
            StatementType::LabelCall(l) => write!(f, "LABEL CALL: {}", l),
            StatementType::BuiltinFunctionCall(b) => {
                write!(f, "BUILTIN FUNCTION CALL: {:?}", b.kind)
            }
        }
    }
}
