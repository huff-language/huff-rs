use crate::evm::Opcode;

type Literal = [u8; 32];

/// A File Path
///
/// Used for parsing the huff imports.
pub type FilePath<'a> = &'a str;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl MacroDefinition<'_> {
    /// Public associated function that instantiates a MacroDefinition.
    pub fn new(
        name: String,
        parameters: Vec<Argument>,
        statements: Vec<Statement<'static>>,
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
    pub args: Vec<&'a Literal>,
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
}
