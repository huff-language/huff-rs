use crate::bytes_util::*;
use ethers_core::abi::{ethereum_types::*, token::*, Tokenizable};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};

/// Primitive EVM types
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            // Default to 256 if no size
            let size = match input.get(4..input.len()) {
                Some(s) => match s.is_empty() {
                    false => match s.parse::<usize>() {
                        Ok(s) => s,
                        Err(_) => return Err(format!("Invalid uint size : {s}")),
                    },
                    true => 256,
                },
                None => 256,
            };
            return Ok(PrimitiveEVMType::Uint(size))
        }
        if input.starts_with("int") {
            // Default to 256 if no size
            let size = match input.get(3..input.len()) {
                Some(s) => match s.is_empty() {
                    false => match s.parse::<usize>() {
                        Ok(s) => s,
                        Err(_) => return Err(format!("Invalid int size : {s}")),
                    },
                    true => 256,
                },
                None => 256,
            };
            return Ok(PrimitiveEVMType::Int(size))
        }
        if input.starts_with("bytes") && input.len() != 5 {
            let remaining = input.get(5..input.len()).unwrap();
            let size = match remaining.parse::<usize>() {
                Ok(s) => s,
                Err(_) => return Err(format!("Invalid bytes size : {remaining}")),
            };
            return Ok(PrimitiveEVMType::Bytes(size))
        }
        if input.eq("bool") {
            return Ok(PrimitiveEVMType::Bool)
        }
        if input.eq("address") {
            return Ok(PrimitiveEVMType::Address)
        }
        if input.eq("string") {
            return Ok(PrimitiveEVMType::String)
        }
        if input == "bytes" {
            Ok(PrimitiveEVMType::DynBytes)
        } else {
            Err(format!("Invalid PrimitiveEVMType type: {input}"))
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
            PrimitiveEVMType::Uint(s) => return write!(f, "uint{s}"),
            PrimitiveEVMType::Int(s) => return write!(f, "int{s}"),
            PrimitiveEVMType::Bytes(s) => return write!(f, "bytes{s}"),
        };

        write!(f, "{x}")
    }
}

// Array of regex to matching fancier EVM types
lazy_static! {
    /// Array of regex to matching fancier EVM types
    pub static ref EVM_TYPE_ARRAY_REGEX: Regex = Regex::new(r"((u|)int[0-9]*|address|bool|bytes|string|bytes[0-9]*)\[[0-9]*\]").unwrap();
}

/// Wrap ether-rs Token to allow to derive the TryFrom trait
#[derive(Clone)]
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
                    42 => {
                        return Ok(EToken(Token::Address(
                            H160::from_str(cleaned_input).map_err(|e| e.to_string())?,
                        )))
                    }
                    _ => {
                        return Ok(EToken(Token::FixedBytes(str_to_bytes32(cleaned_input).to_vec())))
                    }
                }
            } else {
                // dyn bytes array
                return Ok(EToken(Token::Bytes(
                    str_to_vec(cleaned_input).map_err(|e| e.to_string())?,
                )))
            }
        }
        // array
        if input.starts_with('[') {
            let trimmed_input = input.trim_start_matches('[').trim_end_matches(']');
            let v: Vec<String> =
                trimmed_input.split(',').map(|x| x.replace([' ', '"', '\''], "")).collect();
            let etokens: Result<Vec<EToken>, _> =
                v.iter().map(|x| EToken::try_from(x.to_owned())).collect();
            let tokens: Vec<Token> = etokens?.iter().map(move |x| x.clone().0).collect();
            return Ok(EToken(Token::Array(tokens)))
        }
        if input.starts_with('-') || input.starts_with('+') {
            return Ok(EToken(input.parse::<i128>().map_err(|e| e.to_string())?.into_token()))
        }
        if input == "true" || input == "false" {
            return Ok(EToken(Token::Bool(input == "true")))
        }
        if input.chars().all(|x| x.is_ascii_digit()) {
            return Ok(EToken(Token::Uint(
                U256::from_str_radix(input.as_str(), 10).map_err(|e| e.to_string())?,
            )))
        }
        if input.chars().all(|x| x.is_alphanumeric()) {
            Ok(EToken(Token::String(input)))
        } else if input.contains(',') {
            // Try to unwrap something like "100,0x123,20" without brackets
            let e_tokens: Result<Vec<EToken>, _> = input
                .split(',')
                .map(|x| x.replace([' ', '"', '\''], ""))
                .map(EToken::try_from)
                .collect();
            let tokens: Vec<Token> = e_tokens?.into_iter().map(|x| x.0).collect();
            Ok(EToken(Token::Array(tokens)))
        } else {
            Err(format!("Invalid input: {input}"))
        }
    }
}
