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
//! use std::sync::{Arc, Mutex};
//! use huff_utils::prelude::*;
//!
//! // Generate a default contract for demonstrative purposes.
//! // Realistically, contract generation would be done as shown in [huff_parser](./huff_parser)
//! let contract = Contract {
//!     macros: vec![],
//!     invocations: vec![],
//!     imports: vec![],
//!     constants: Arc::new(Mutex::new(vec![])),
//!     errors: vec![],
//!     functions: vec![huff_utils::ast::FunctionDefinition {
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
//! ```

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt};

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
    /// A list of errors and their definitions
    pub errors: BTreeMap<String, Error>,
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
        // Try to get the constructor inputs from an overriden function
        // Otherwise, use the CONSTRUCTOR macro if one exists
        let constructor = contract
            .functions
            .iter()
            .filter(|m| m.name.to_lowercase() == "constructor")
            .cloned()
            .collect::<Vec<ast::FunctionDefinition>>()
            .get(0)
            .map(|func| Constructor {
                inputs: func
                    .inputs
                    .iter()
                    .map(|argument| FunctionParam {
                        name: argument.name.clone().unwrap_or_default(),
                        kind: argument.arg_type.clone().unwrap_or_default().into(),
                        internal_type: None,
                    })
                    .collect(),
            })
            .or_else(|| {
                contract
                    .macros
                    .iter()
                    .filter(|m| m.name == "CONSTRUCTOR")
                    .cloned()
                    .collect::<Vec<ast::MacroDefinition>>()
                    .get(0)
                    .map(|func| Constructor {
                        inputs: func
                            .parameters
                            .iter()
                            .map(|argument| FunctionParam {
                                name: argument.name.clone().unwrap_or_default(),
                                kind: argument.name.clone().unwrap_or_default().into(),
                                internal_type: None,
                            })
                            .collect(),
                    })
            });

        // Instantiate functions and events
        let mut functions = BTreeMap::new();
        let mut events = BTreeMap::new();
        let mut errors = BTreeMap::new();

        // Translate contract functions
        // Excluding constructor
        functions.extend(
            contract
                .functions
                .iter()
                .filter(|function: &&ast::FunctionDefinition| function.name != "CONSTRUCTOR")
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
                }),
        );

        // Translate contract events
        events.extend(contract.events.iter().map(|event| {
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
                            indexed: argument.indexed,
                        })
                        .collect(),
                    anonymous: false,
                },
            )
        }));

        // Translate contract errors
        errors.extend(contract.errors.iter().map(|error| {
            (
                error.name.to_string(),
                Error {
                    name: error.name.to_string(),
                    inputs: error
                        .parameters
                        .iter()
                        .map(|argument| FunctionParam {
                            name: argument.name.clone().unwrap_or_default(),
                            kind: argument.arg_type.clone().unwrap_or_default().into(),
                            internal_type: None,
                        })
                        .collect(),
                },
            )
        }));

        Self { constructor, functions, events, errors, receive: false, fallback: false }
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

/// #### Error
///
/// An Error definition.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Error {
    /// The error name
    pub name: String,
    /// The error inputs
    pub inputs: Vec<FunctionParam>,
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
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
    /// Array ; uint256[2][] => Array(Uint(256), [2, 0])
    Array(Box<FunctionParamType>, Vec<usize>),
    /// Fixed number of bytes
    FixedBytes(usize),
    /// A tuple of parameters
    Tuple(Vec<FunctionParamType>),
}

impl FunctionParamType {
    /// Checks if the param type should be designated as "memory" for solidity interface
    /// generation.
    pub fn is_memory_type(&self) -> bool {
        matches!(
            self,
            FunctionParamType::Bytes |
                FunctionParamType::String |
                FunctionParamType::Tuple(_) |
                FunctionParamType::Array(_, _)
        )
    }
}

impl fmt::Debug for FunctionParamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}

impl fmt::Display for FunctionParamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}

impl FunctionParamType {
    /// Print a function parameter type to a formatter
    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FunctionParamType::Address => write!(f, "address"),
            FunctionParamType::Bytes => write!(f, "bytes"),
            FunctionParamType::Int(size) => write!(f, "int{size}"),
            FunctionParamType::Uint(size) => write!(f, "uint{size}"),
            FunctionParamType::Bool => write!(f, "bool"),
            FunctionParamType::String => write!(f, "string"),
            FunctionParamType::Array(fpt, sizes) => write!(
                f,
                "{}{}",
                fpt,
                sizes
                    .iter()
                    .map(|s| (!s.eq(&0))
                        .then(|| format!("[{s}]"))
                        .unwrap_or_else(|| "[]".to_string()))
                    .collect::<Vec<_>>()
                    .join("")
            ),
            FunctionParamType::FixedBytes(size) => write!(f, "bytes{size}"),
            FunctionParamType::Tuple(inner) => write!(
                f,
                "({})",
                inner.iter().map(|fpt| fpt.to_string()).collect::<Vec<_>>().join(", ")
            ),
        }
    }

    /// Convert string to type
    pub fn convert_string_to_type(string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input = string.to_string().to_lowercase();
        let split_input: Vec<&str> = input.split('[').collect();
        if split_input.len() > 1 {
            let mut cleaned: Vec<String> = split_input
                .iter()
                .map(|x| x.replace(']', ""))
                .map(|x| if x.is_empty() { "0".to_owned() } else { x })
                .collect();
            let func_type = FunctionParamType::convert_string_to_type(&cleaned.remove(0))?;
            let sizes: Vec<usize> = cleaned.iter().map(|x| x.parse::<usize>().unwrap()).collect();
            return Ok(Self::Array(Box::new(func_type), sizes))
        }
        if input.starts_with("uint") {
            // Default to 256 if no size
            let size = match input.get(4..input.len()) {
                Some(s) => match s.is_empty() {
                    false => s.parse::<usize>().unwrap(),
                    true => 256,
                },
                None => 256,
            };
            return Ok(Self::Uint(size))
        }
        if input.starts_with("int") {
            // Default to 256 if no size
            let size = match input.get(3..input.len()) {
                Some(s) => match s.is_empty() {
                    false => s.parse::<usize>().unwrap(),
                    true => 256,
                },
                None => 256,
            };
            return Ok(Self::Int(size))
        }
        if input.starts_with("bytes") && input.len() != 5 {
            let size = input.get(5..input.len()).unwrap().parse::<usize>().unwrap();
            return Ok(Self::FixedBytes(size))
        }
        if input.starts_with("bool") {
            return Ok(Self::Bool)
        }
        if input.starts_with("address") {
            return Ok(Self::Address)
        }
        if input.starts_with("string") {
            return Ok(Self::String)
        }
        if input == "bytes" {
            Ok(Self::Bytes)
        } else {
            tracing::error!("Failed to create FunctionParamType from string: {}", string);
            Err(format!("Failed to create FunctionParamType from string: {string}"))?
        }
    }
}

impl From<&str> for FunctionParamType {
    fn from(string: &str) -> Self {
        FunctionParamType::convert_string_to_type(string).unwrap()
    }
}

impl From<String> for FunctionParamType {
    fn from(string: String) -> Self {
        FunctionParamType::convert_string_to_type(&string).unwrap()
    }
}
