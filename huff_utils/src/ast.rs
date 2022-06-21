use serde::{Deserialize, Serialize};

use crate::{bytecode::*, bytes_util::*, error::CodegenError, evm::Opcode};
use std::path::PathBuf;

/// A contained literal
pub type Literal = [u8; 32];

/// A File Path
///
/// Used for parsing the huff imports.
pub type FilePath = PathBuf;

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
    /// Functions
    pub functions: Vec<Function>,
    /// Events
    pub events: Vec<Event>,
    /// Tables
    pub tables: Vec<Table>,
}

impl Contract {
    /// Returns the first macro that matches the provided name
    pub fn find_macro_by_name(&self, name: &str) -> Option<MacroDefinition> {
        if let Some(m) = self
            .macros
            .iter()
            .filter(|m| m.name == name)
            .cloned()
            .collect::<Vec<MacroDefinition>>()
            .get(0)
        {
            Some(m.clone())
        } else {
            tracing::warn!("Failed to find macro \"{}\" in contract", name);
            None
        }
    }

    /// Derives the FreeStoragePointers into their bytes32 representation
    pub fn derive_storage_pointers(&mut self) {
        let mut storage_pointers: Vec<[u8; 32]> = Vec::new();
        let mut last_assigned_free_pointer = 0;
        // do the non fsp consts first, so we can check for conflicts
        // when going over the fsp consts
        for constant in &self.constants {
            if let ConstVal::Literal(literal) = &constant.value {
                storage_pointers.push(*literal);
            }
        }
        for constant in self.constants.iter_mut() {
            if let ConstVal::FreeStoragePointer(_) = &constant.value {
                let mut fsp_bytes = str_to_bytes32(&format!("{}", last_assigned_free_pointer));
                while storage_pointers.contains(&fsp_bytes) {
                    last_assigned_free_pointer += 1;
                    fsp_bytes = str_to_bytes32(&format!("{}", last_assigned_free_pointer));
                }
                storage_pointers.push(fsp_bytes);
                last_assigned_free_pointer += 1;
                constant.value = ConstVal::Literal(fsp_bytes);
            }
        }
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

/// An Event Signature
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    /// The name of the event
    pub name: String,
    /// The parameters of the event
    pub parameters: Vec<Argument>,
}

/// A Table Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    /// The name of the table
    pub name: String,
    // TODO:::
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
}

impl<'a> ToIRBytecode<CodegenError<'a>> for MacroDefinition {
    fn to_irbytecode(&self) -> Result<IRBytecode, CodegenError<'a>> {
        let mut inner_irbytes: Vec<IRByte> = vec![];

        // Iterate and translate each statement to bytecode
        self.statements.iter().for_each(|statement| {
            match statement {
                Statement::Literal(l) => {
                    let combined = l
                        .iter()
                        .map(|b| IRByte::Byte(Byte(format!("{:04x}", b))))
                        .collect::<Vec<IRByte>>();
                    tracing::info!(target: "codegen", "COMBINED IRBytes: {:?}", combined);
                    combined.iter().for_each(|irb| inner_irbytes.push(irb.clone()));
                }
                Statement::Opcode(o) => {
                    let opcode_str = o.string();
                    tracing::info!("Got opcode hex string: {}", opcode_str);
                    inner_irbytes.push(IRByte::Byte(Byte(opcode_str)))
                }
                Statement::MacroInvocation(mi) => {
                    inner_irbytes.push(IRByte::Statement(Statement::MacroInvocation(mi.clone())));
                }
                Statement::Constant(name) => {
                    // Constant needs to be evaluated at the top-level
                    inner_irbytes.push(IRByte::Constant(name.to_string()));
                }
                Statement::ArgCall(arg_name) => {
                    // Arg call needs to use a destination defined in the calling macro context
                    inner_irbytes.push(IRByte::ArgCall(arg_name.to_string()));
                }
            }
        });
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
    ) -> Self {
        MacroDefinition { name, parameters, statements, takes, returns }
    }
}

/// A Macro Invocation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacroInvocation {
    /// The Macro Name
    pub macro_name: String,
    /// A list of Macro arguments
    pub args: Vec<MacroArg>,
}

/// An argument passed when invoking a maco
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MacroArg {
    /// Macro Literal Argument
    Literal(Literal),
    /// Macro Iden String Argument
    Ident(String),
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
}

/// A Statement
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Statement {
    /// A Literal Statement
    Literal(Literal),
    /// An Opcode Statement
    Opcode(Opcode),
    /// A Macro Invocation Statement
    MacroInvocation(MacroInvocation),
    /// A Constant Push
    Constant(String),
    /// An Arg Call
    ArgCall(String),
}
