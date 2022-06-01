//! ## Abi
//!
//! A module containing ABI type definitions for ethereum contracts.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// #### Abi
///
/// The ABI of the generated code.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Abi {
    /// The constructor
    pub constructor: Option<Constructor>,
    /// A list of functions and their definitions
    pub functions: BTreeMap<String, Vec<Function>>,
    /// A list of events and their definitions
    pub events: BTreeMap<String, Vec<Event>>,
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

/// #### Function
///
/// A function definition.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
    pub state_mutability: StateMutability,
}

/// #### StateMutability
///
/// The state mutability.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum StateMutability {
    /// Doesn't read state
    Pure,
    /// Only reads state, no writes
    View,
    /// Not Payable
    NonPayable,
    /// Payable
    Payable,
}

/// #### Event
///
/// An Event definition.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Constructor {
    /// Contstructor inputs
    pub inputs: Vec<FunctionParam>,
}

/// #### FunctionParam
///
/// A generic function parameter
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
