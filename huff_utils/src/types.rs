use lazy_static::lazy_static;
use regex::Regex;

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
            return PrimitiveEVMType::Uint(size);
        }
        if input.starts_with("int") {
            let size = input.get(3..input.len()).unwrap().parse::<usize>().unwrap();
            return PrimitiveEVMType::Int(size);
        }
        if input.starts_with("bytes") && input.len() != 5 {
            let size = input.get(5..input.len()).unwrap().parse::<usize>().unwrap();
            return PrimitiveEVMType::Bytes(size);
        }
        if input.starts_with("bool") {
            return PrimitiveEVMType::Bool;
        }
        if input.starts_with("address") {
            return PrimitiveEVMType::Address;
        }
        if input.starts_with("string") {
            return PrimitiveEVMType::String;
        }
        if input == "bytes" {
            return PrimitiveEVMType::DynBytes;
        } else {
            panic!("Invalid PrimitiveEVMType type: {}", input);
        }
    }
}

/// All the valid primitive EVM types.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EVMType {
    /// Primitive EVM type
    Primitive(PrimitiveEVMType),
    /// Array ; can be of any type, bounded or not
    /// a size of 0 indicates unbounded
    Array(PrimitiveEVMType, usize)
}

impl From<String> for EVMType {
    fn from(input: String) -> Self {
        // is uint?
        if input.starts_with("uint") {
            let size = input.get(4..input.len()).unwrap().parse::<usize>().unwrap();
            return EVMType::Primitive(PrimitiveEVMType::Uint(size));
        }
        if input.starts_with("int") {
            let size = input.get(3..input.len()).unwrap().parse::<usize>().unwrap();
            return EVMType::Primitive(PrimitiveEVMType::Int(size));
        }
        if input.starts_with("bytes") && input.len() != 5 {
            let size = input.get(5..input.len()).unwrap().parse::<usize>().unwrap();
            return EVMType::Primitive(PrimitiveEVMType::Bytes(size));
        }
        if input.starts_with("bool") {
            return EVMType::Primitive(PrimitiveEVMType::Bool);
        }
        if input.starts_with("address") {
            return EVMType::Primitive(PrimitiveEVMType::Address);
        }
        if input.starts_with("string") {
            return EVMType::Primitive(PrimitiveEVMType::String);
        }
        if input == "bytes" {
            return EVMType::Primitive(PrimitiveEVMType::DynBytes);
        } else {
            panic!("Invalid EVM type: {}", input);
        }
    }
}


/// Array of regex to matching fancier EVM types
lazy_static! {
    /// Array of regex to matching fancier EVM types
    pub static ref EVMTypeArrayRegex: Regex = Regex::new(r"((u|)int[0-9]*|address|bool|bytes|string|bytes[0-9]*)\[[0-9]*\]").unwrap();
}
