use crate::evm::Opcode;

/// A Huff literal
///
///32-byte EVM word
pub type Literal = [u8; 32];

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
pub struct Contract<'a> {
    /// Macro definitions
    pub macros: Vec<MacroDefinition<'a>>,
    /// Invocations of macros
    pub invocations: Vec<MacroInvocation<'a>>,
    /// File Imports
    pub imports: Vec<FilePath<'a>>,
    /// Constants
    pub constants: Vec<Constant<'a>>,
    /// Functions
    pub functions: Vec<Function<'a>>,
    /// Events
    pub events: Vec<Event<'a>>,
    /// Tables
    pub tables: Vec<Table<'a>>,
}

/// A Constant Definition
pub struct Constant<'a> {
    /// The name of the constant
    pub name: &'a str,
    /// The literal value of the constant
    pub literal: Literal,
}

/// A Function Signature
pub struct Function<'a> {
    /// The name of the function
    pub name: &'a str,
    /// The parameters of the function
    pub parameters: Vec<String>,
    /// The function decorators
    pub decorators: Vec<String>,
    /// The return type of the function
    pub return_type: String,
}

/// An Event Signature
pub struct Event<'a> {
    /// The name of the event
    pub name: &'a str,
    /// The parameters of the event
    pub parameters: Vec<String>,
}

/// A Table Definition
pub struct Table<'a> {
    /// The name of the table
    pub name: &'a str,
    // TODO:::
}

/// A Macro Definition
#[derive(Debug)]
pub struct MacroDefinition<'a> {
    /// The Macro Name
    pub name: String,
    /// A list of Macro parameters
    pub parameters: Vec<String>,
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
        parameters: Vec<String>,
        statements: Vec<Statement<'static>>,
        takes: usize,
        returns: usize,
    ) -> Self {
        MacroDefinition { name, parameters, statements, takes, returns }
    }
}

/// A Macro Invocation
#[derive(Debug)]
pub struct MacroInvocation<'a> {
    /// The Macro Name
    pub macro_name: String,
    /// A list of Macro arguments
    pub args: Vec<&'a Literal>,
}

impl<'a> MacroInvocation<'a> {
    /// Public associated function that instantiates a MacroDefinition.
    pub fn new(
        macro_name: String,
        args: Vec<&'a Literal>
    ) -> Self {
        MacroInvocation {
            macro_name,
            args,
        }
    }
}

/// A Constant
pub struct ConstantDefinition<'a> {
    /// The Constant name
    pub name: &'a str,
    /// The Constant value
    pub value: Literal,
}

/// A Statement
#[derive(Debug)]
pub enum Statement<'a> {
    /// A Literal Statement
    Literal(Literal),
    /// An Opcode Statement
    Opcode(Opcode),
    /// A Macro Invocation Statement
    MacroInvocation(MacroInvocation<'a>),
}