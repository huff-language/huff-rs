use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
/// Primitive EVM types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveEVMType {
    /// String type
    String,
    /// Bytes array type
    DynBytes,
    /// Boolean type
    Bool,
    /// Uint ; should start from uint8 to uint256 by step of 8
    Uint(usize),
    /// Int ; should start from int8 to int256 by step of 8
    Int(usize),
    /// Address type
    Address,
    /// Bytes type ; bytes1, bytes2, ..., bytes32
    Bytes(usize),
}

/// Automatically converts an input string to a PrimitiveEVMType.
/// Example : PrimitiveEVMType::from("uint256") => PrimitiveEVMType::Uint(256)
impl From<String> for PrimitiveEVMType {
    fn from(input: String) -> Self {
        if input.starts_with("uint") {
            let size = input.get(4..input.len()).unwrap().parse::<usize>().unwrap();
            return PrimitiveEVMType::Uint(size)
        }
        if input.starts_with("int") {
            let size = input.get(3..input.len()).unwrap().parse::<usize>().unwrap();
            return PrimitiveEVMType::Int(size)
        }
        if input.starts_with("bytes") && input.len() != 5 {
            let size = input.get(5..input.len()).unwrap().parse::<usize>().unwrap();
            return PrimitiveEVMType::Bytes(size)
        }
        if input.starts_with("bool") {
            return PrimitiveEVMType::Bool
        }
        if input.starts_with("address") {
            return PrimitiveEVMType::Address
        }
        if input.starts_with("string") {
            return PrimitiveEVMType::String
        }
        if input == "bytes" {
            PrimitiveEVMType::DynBytes
        } else {
            panic!("Invalid PrimitiveEVMType type: {}", input);
        }
    }
}

impl fmt::Display for PrimitiveEVMType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            PrimitiveEVMType::Address => "address",
            PrimitiveEVMType::String => "string",
            PrimitiveEVMType::Bool => "bool",
            PrimitiveEVMType::DynBytes => "bytes",
            PrimitiveEVMType::Uint(s) => return write!(f, "uint{}", s),
            PrimitiveEVMType::Int(s) => return write!(f, "int{}", s),
            PrimitiveEVMType::Bytes(s) => return write!(f, "bytes{}", s),
        };

        write!(f, "{}", x)
    }
}

// Array of regex to matching fancier EVM types
lazy_static! {
    /// Array of regex to matching fancier EVM types
    pub static ref EVM_TYPE_ARRAY_REGEX: Regex = Regex::new(r"((u|)int[0-9]*|address|bool|bytes|string|bytes[0-9]*)\[[0-9]*\]").unwrap();
}
