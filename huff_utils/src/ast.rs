use serde::{Deserialize, Serialize};

use crate::{bytecode::*, error::CodegenError, evm::Opcode};
use std::path::Path;

type Literal = [u8; 32];

/// A File Path
///
/// Used for parsing the huff imports.
pub type FilePath<'a> = &'a Path;

/// A Huff Contract Representation
///
/// This is the representation of a contract as it is parsed from huff source code.
/// Thus, it is also the root of the AST.
///
/// For examples of Huff contracts, see the [huff-examples repository](https://github.com/huff-language/huff-examples).
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Contract<'a> {
    /// Macro definitions
    pub macros: Vec<MacroDefinition<'a>>,
    /// Invocations of macros
    pub invocations: Vec<MacroInvocation<'a>>,
    /// File Imports
    pub imports: Vec<FilePath<'a>>,
    /// Constants
    pub constants: Vec<ConstantDefinition<'a>>,
    /// Functions
    pub functions: Vec<Function<'a>>,
    /// Events
    pub events: Vec<Event<'a>>,
    /// Tables
    pub tables: Vec<Table<'a>>,
}

impl<'a> Contract<'a> {
    /// Returns the first macro that matches the provided name
    pub fn find_macro_by_name(&self, name: &'a str) -> Option<MacroDefinition<'a>> {
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
pub struct Function<'a> {
    /// The name of the function
    pub name: &'a str,
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
pub struct Event<'a> {
    /// The name of the event
    pub name: &'a str,
    /// The parameters of the event
    pub parameters: Vec<Argument>,
}

/// A Table Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table<'a> {
    /// The name of the table
    pub name: &'a str,
    // TODO:::
}

/// A Macro Definition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacroDefinition<'a> {
    /// The Macro Name
    pub name: String,
    /// A list of Macro parameters
    pub parameters: Vec<Argument>,
    /// A list of Statements contained in the Macro
    pub statements: Vec<Statement<'a>>,
    /// The take size
    pub takes: usize,
    /// The return size
    pub returns: usize,
}

impl<'a> ToIRBytecode<'a, CodegenError<'a>> for MacroDefinition<'a> {
    fn to_irbytecode(&self) -> Result<IRBytecode<'a>, CodegenError<'a>> {
        let mut inner_irbytes: Vec<IRByte> = vec![];

        // Iterate and translate each statement to bytecode
        self.statements.iter().for_each(|statement| {
            match statement {
                Statement::Literal(l) => {
                    let combined = l
                        .iter()
                        .map(|b| IRByte::Byte(Byte(format!("{:04x}", b))))
                        .collect::<Vec<IRByte>>();
                    println!("Combined IRBytes: {:?}", combined);
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
                    inner_irbytes.push(IRByte::Constant(name));
                }
            }
        });
        Ok(IRBytecode(inner_irbytes))
    }
}

impl<'a> MacroDefinition<'a> {
    /// Public associated function that instantiates a MacroDefinition.
    pub fn new(
        name: String,
        parameters: Vec<Argument>,
        statements: Vec<Statement<'a>>,
        takes: usize,
        returns: usize,
    ) -> Self {
        MacroDefinition { name, parameters, statements, takes, returns }
    }
}

/// A Macro Invocation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacroInvocation<'a> {
    /// The Macro Name
    pub macro_name: String,
    /// A list of Macro arguments
    pub args: Vec<MacroArg<'a>>,
}

/// An argument passed when invoking a maco
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MacroArg<'a> {
    /// Macro Literal Argument
    Literal(Literal),
    /// Macro Iden String Argument
    Ident(&'a str),
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
pub struct ConstantDefinition<'a> {
    /// The Constant name
    pub name: &'a str,
    /// The Constant value
    pub value: ConstVal,
}

/// A Statement
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Statement<'a> {
    /// A Literal Statement
    Literal(Literal),
    /// An Opcode Statement
    Opcode(Opcode),
    /// A Macro Invocation Statement
    MacroInvocation(MacroInvocation<'a>),
    /// A Constant Push
    Constant(&'a str),
}
