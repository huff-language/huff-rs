use crate::bytes_util::*;
use ethers::abi::{ethereum_types::*, token::Token};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};
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
impl TryFrom<String> for PrimitiveEVMType {
    type Error = String;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.starts_with("uint") {
            let size = input.get(4..input.len()).unwrap().parse::<usize>().unwrap();
            return Ok(PrimitiveEVMType::Uint(size))
        }
        if input.starts_with("int") {
            let size = input.get(3..input.len()).unwrap().parse::<usize>().unwrap();
            return Ok(PrimitiveEVMType::Int(size))
        }
        if input.starts_with("bytes") && input.len() != 5 {
            let size = input.get(5..input.len()).unwrap().parse::<usize>().unwrap();
            return Ok(PrimitiveEVMType::Bytes(size))
        }
        if input.starts_with("bool") {
            return Ok(PrimitiveEVMType::Bool)
        }
        if input.starts_with("address") {
            return Ok(PrimitiveEVMType::Address)
        }
        if input.starts_with("string") {
            return Ok(PrimitiveEVMType::String)
        }
        if input == "bytes" {
            Ok(PrimitiveEVMType::DynBytes)
        } else {
            Err(format!("Invalid PrimitiveEVMType type: {}", input))
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

/// Wrap ether-rs Token to allow to derive the TryFrom trait
pub struct EToken(pub Token);

impl TryFrom<String> for EToken {
    type Error = String;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.starts_with("0x") {
            // remove 0x prefix
            let cleaned_input = input.get(2..input.len()).unwrap();
            // either address or fixed bytes
            if cleaned_input.len() <= 64 {
                // could be either address or fixed bytes
                // if length is 42, assume it's an address
                match input.len() {
                    42 => return Ok(EToken(Token::Address(H160::from_str(cleaned_input).unwrap()))),
                    _ => {
                        return Ok(EToken(Token::FixedBytes(str_to_bytes32(cleaned_input).to_vec())))
                    }
                }
            } else {
                // dyn bytes array
                return Ok(EToken(Token::Bytes(str_to_vec(cleaned_input))))
            }
        }
        if input == "true" || input == "false" {
            return Ok(EToken(Token::Bool(input == "true")))
        }
        if input.chars().all(|x| x.is_ascii_digit()) {
            return Ok(EToken(Token::Uint(U256::from_str_radix(input.as_str(), 10).unwrap())))
        }
        if input.chars().all(|x| x.is_alphanumeric()) {
            Ok(EToken(Token::String(input)))
        } else {
            Err(format!("Invalid input: {}", input))
        }
    }
}
