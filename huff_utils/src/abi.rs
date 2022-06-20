//! ## Abi
//!
//! A module containing ABI type definitions for ethereum contracts.
//!
//! #### Usage
//!
//! Let's say you have a `Contract<'a>` generated from the [huff_parser](./huff_parser),
//! representing an AST. This crate let's you easily convert the `Contract<'a>` into an `Abi`
//! instance like so:
//!
//! ```rust
//! use huff_utils::prelude::*;
//!
//! // Generate a default contract for demonstrative purposes.
//! // Realistically, contract generation would be done as shown in [huff_parser](./huff_parser)
//! let contract = Contract {
//!     macros: vec![],
//!     invocations: vec![],
//!     imports: vec![],
//!     constants: vec![],
//!     functions: vec![huff_utils::ast::Function {
//!         name: "CONSTRUCTOR".to_string(),
//!         signature: [0u8, 0u8, 0u8, 0u8],
//!         inputs: vec![],
//!         fn_type: FunctionType::NonPayable,
//!         outputs: vec![],
//!         span: AstSpan(vec![]),
//!     }],
//!     events: vec![],
//!     tables: vec![],
//! };
//!
//! // Create an ABI using that generate contract
//! let abi: Abi = contract.into();
//! println!("Abi instant: {:?}", abi);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::ast::{self, FunctionType};

/// #### Abi
///
/// The ABI of the generated code.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Abi {
    /// The constructor
    pub constructor: Option<Constructor>,
    /// A list of functions and their definitions
    pub functions: BTreeMap<String, Function>,
    /// A list of events and their definitions
    pub events: BTreeMap<String, Event>,
    /// If the contract defines receive logic
    pub receive: bool,
    /// If the contract defines fallback logic
    pub fallback: bool,
}

impl Abi {
    /// Public associated function to instatiate a new Abi.
    pub fn new() -> Self {
        Self::default()
    }
}

// Allows for simple ABI Generation by directly translating the AST
impl From<ast::Contract> for Abi {
    fn from(contract: ast::Contract) -> Self {
        let constructors = contract
            .macros
            .iter()
            .filter(|m| m.name == "CONSTRUCTOR")
            .cloned()
            .collect::<Vec<ast::MacroDefinition>>();
        let constructor: Option<&ast::MacroDefinition> = constructors.get(0);

        // Instantiate functions and events
        let mut functions = BTreeMap::new();
        let mut events = BTreeMap::new();

        // Translate contract functions
        // Excluding constructor
        contract
            .functions
            .iter()
            .filter(|function: &&ast::Function| function.name != "CONSTRUCTOR")
            .map(|function| {
                (
                    function.name.to_string(),
                    Function {
                        name: function.name.to_string(),
                        inputs: function
                            .inputs
                            .iter()
                            .map(|argument| FunctionParam {
                                name: argument.name.clone().unwrap_or_default(),
                                kind: argument.arg_type.clone().unwrap_or_default().into(),
                                internal_type: None,
                            })
                            .collect(),
                        outputs: function
                            .outputs
                            .iter()
                            .map(|argument| FunctionParam {
                                name: argument.name.clone().unwrap_or_default(),
                                kind: argument.arg_type.clone().unwrap_or_default().into(),
                                internal_type: None,
                            })
                            .collect(),
                        constant: false,
                        state_mutability: function.fn_type.clone(),
                    },
                )
            })
            .for_each(|val| {
                let _ = functions.insert(val.0, val.1);
            });

        // Translate contract events
        contract
            .events
            .iter()
            .map(|event| {
                (
                    event.name.to_string(),
                    Event {
                        name: event.name.to_string(),
                        inputs: event
                            .parameters
                            .iter()
                            .map(|argument| EventParam {
                                name: argument.name.clone().unwrap_or_default(),
                                kind: argument.arg_type.clone().unwrap_or_default().into(),
                                indexed: false, // TODO: This is not present in `argument`
                            })
                            .collect(),
                        anonymous: false,
                    },
                )
            })
            .for_each(|val| {
                let _ = events.insert(val.0, val.1);
            });

        Self {
            constructor: constructor.map(|c| Constructor {
                inputs: c
                    .parameters
                    .iter()
                    .map(|argument| FunctionParam {
                        name: argument.name.clone().unwrap_or_default(),
                        kind: argument.arg_type.clone().unwrap_or_default().into(),
                        internal_type: None,
                    })
                    .collect(),
            }),
            functions,
            events,
            receive: false,
            fallback: false,
        }
    }
}

/// #### Function
///
/// A function definition.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Function {
    /// The function name
    pub name: String,
    /// The function inputs
    pub inputs: Vec<FunctionParam>,
    /// The function outputs
    pub outputs: Vec<FunctionParam>,
    /// Constant
    pub constant: bool,
    /// The state mutability
    pub state_mutability: FunctionType,
}

/// #### Event
///
/// An Event definition.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Event {
    /// The event name
    pub name: String,
    /// The event inputs
    pub inputs: Vec<EventParam>,
    /// Anonymity
    pub anonymous: bool,
}

/// #### EventParam
///
/// Event parameters.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct EventParam {
    /// The parameter name
    pub name: String,
    /// The parameter type
    pub kind: FunctionParamType,
    /// If the parameter is indexed
    pub indexed: bool,
}

/// #### Constructor
///
/// The contract constructor
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Constructor {
    /// Contstructor inputs
    pub inputs: Vec<FunctionParam>,
}

/// #### FunctionParam
///
/// A generic function parameter
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FunctionParam {
    /// The function parameter name
    pub name: String,
    /// The function parameter type
    pub kind: FunctionParamType,
    /// The internal type of the parameter
    pub internal_type: Option<String>,
}

/// #### FunctionParamType
///
/// The type of a function parameter
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum FunctionParamType {
    /// An address
    Address,
    /// Bytes
    Bytes,
    /// A signed integer
    Int(usize),
    /// An unsigned integer
    Uint(usize),
    /// A boolean
    Bool,
    /// A String
    String,
    /// An array of parameters
    Array(Box<FunctionParamType>),
    /// Fixed number of bytes
    FixedBytes(usize),
    /// Fixed size array of parameters
    FixedArray(Box<FunctionParamType>, usize),
    /// A tuple of parameters
    Tuple(Vec<FunctionParamType>),
}

impl From<&str> for FunctionParamType {
    fn from(string: &str) -> Self {
        match string {
            "Address" | "address" => Self::Address,
            "Bytes" | "bytes" => Self::Bytes,
            "Int" | "int" | "integer" | "Integer" => Self::Int(0),
            "Uint" | "uint" | "unsignedinteger" | "unsigned integer" => Self::Uint(0),
            "Bool" | "bool" => Self::Bool,
            "String" | "string" | "str" | "Str" => Self::String,
            "Array" | "array" => Self::Array(Box::new(FunctionParamType::Bool)),
            "FixedBytes" | "bytes32" => Self::Array(Box::new(FunctionParamType::Bool)),
            _ => {
                tracing::error!(
                    "{}",
                    format!("Failed to create FunctionParamType from string: {}", string)
                );
                panic!("{}", format!("Failed to create FunctionParamType from string: {}", string))
            }
        }
    }
}

impl From<String> for FunctionParamType {
    fn from(string: String) -> Self {
        match string.as_ref() {
            "Address" | "address" => Self::Address,
            "Bytes" | "bytes" => Self::Bytes,
            "Int" | "int" | "integer" | "Integer" => Self::Int(0),
            "Uint" | "uint" | "uint256" | "unsignedinteger" | "unsigned integer" => Self::Uint(0),
            "Bool" | "bool" => Self::Bool,
            "String" | "string" | "str" | "Str" => Self::String,
            "Array" | "array" => Self::Array(Box::new(FunctionParamType::Bool)),
            "FixedBytes" | "bytes32" => Self::Array(Box::new(FunctionParamType::Bool)),
            _ => {
                tracing::error!(
                    target: "abi",
                    "{}",
                    format!("Failed to create FunctionParamType from string: {}", string)
                );
                panic!("{}", format!("Failed to create FunctionParamType from string: {}", string))
            }
        }
    }
}
