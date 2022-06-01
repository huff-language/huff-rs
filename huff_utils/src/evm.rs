use std::fmt;
use strum_macros::EnumString;

/// EVM Opcodes
/// References https://evm.codes
#[derive(Debug, PartialEq, EnumString)]
pub enum Opcode {
    /// Halts execution.
    #[strum(serialize = "stop")]
    Stop,
    /// Addition operation
    #[strum(serialize = "add")]
    Add,
    /// Multiplication Operation
    #[strum(serialize = "mul")]
    Mul,
    /// Subtraction Operation
    Sub,
    /// Integer Division Operation
    Div,
    /// Signed Integer Division Operation
    Sdiv,
    /// Modulo Remainder Operation
    Mod,
    /// Signed Modulo Remainder Operation
    Smod,
    /// Modulo Addition Operation
    Addmod,
    /// Modulo Multiplication Operation
    Mulmod,
    /// Exponential Operation
    Exp,
    /// Extend Length of Two's Complement Signed Integer
    Signextend,
    /// Less-than Comparison
    Lt,
    /// Greater-than Comparison
    Gt,
    /// Signed Less-than Comparison
    Slt,
    /// Signed Greater-than Comparison
    Sgt,
    /// Equality Comparison
    Eq,
    /// Not Operation
    Iszero,
    /// Bitwise AND Operation
    And,
    /// Bitwise OR Operation
    Or,
    /// Bitwise XOR Operation
    Xor,
    /// Bitwise NOT Operation
    Not,
    /// Retrieve Single Byte from Word
    Byte,
    /// Left Shift Operation
    Shl,
    /// Right Shift Operation
    Shr,
    /// Arithmetic Shift Right Operation
    Sar,
    /// Compute the Keccak-256 hash of a 32-byte word
    Sha3,
    /// Address of currently executing account
    Address,
    /// Balance of a given account
    Balance,
    /// Address of execution origination
    Origin,
    /// Address of the caller
    Caller,
    /// Value of the call
    Callvalue,
    /// Loads Calldata
    Calldataload,
    /// Size of the Calldata
    Calldatasize,
    /// Copies the Calldata to Memory
    Calldatacopy,
    /// Size of the Executing Code
    Codesize,
    /// Copies Executing Code to Memory
    Codecopy,
    /// Current Price of Gas
    Gasprice,
    /// Size of an Account's Code
    Extcodesize,
    /// Copies an Account's Code to Memory
    Extcodecopy,
    /// Size of Output Data from Previous Call
    Returndatasize,
    /// Copies Output Data from Previous Call to Memory
    Returndatacopy,
    /// Hash of a Block from the most recent 256 blocks
    Blockhash,
    /// The Current Blocks Beneficiary Address
    Coinbase,
    /// The Current Blocks Timestamp
    Timestamp,
    /// The Current Blocks Number
    Number,
    /// The Current Blocks Difficulty
    Difficulty,
    /// The Current Blocks Gas Limit
    Gaslimit,
    /// The Chain ID
    Chainid,
    /// Balance of the Currently Executing Account
    Selfbalance,
    /// Base Fee
    Basefee,
    /// Removes an Item from the Stack
    Pop,
    /// Loads a word from Memory
    Mload,
    /// Stores a word in Memory
    Mstore,
    /// Stores a byte in Memory
    Mstore8,
    /// Load a word from Storage
    Sload,
    /// Store a word in Storage
    Sstore,
    /// Alter the Program Counter
    Jump,
    /// Conditionally Alter the Program Counter
    Jumpi,
    /// Value of the Program Counter Before the Current Instruction
    Pc,
    /// Size of Active Memory in Bytes
    Msize,
    /// Amount of available gas including the cost of the current instruction
    Gas,
    /// Marks a valid destination for jumps
    Jumpdest,
    /// Places 1 byte item on top of the stack
    Push1,
    /// Places 2 byte item on top of the stack
    Push2,
    /// Places 3 byte item on top of the stack
    Push3,
    /// Places 4 byte item on top of the stack
    Push4,
    /// Places 5 byte item on top of the stack
    Push5,
    /// Places 6 byte item on top of the stack
    Push6,
    /// Places 7 byte item on top of the stack
    Push7,
    /// Places 8 byte item on top of the stack
    Push8,
    /// Places 9 byte item on top of the stack
    Push9,
    /// Places 10 byte item on top of the stack
    Push10,
    /// Places 11 byte item on top of the stack
    Push11,
    /// Places 12 byte item on top of the stack
    Push12,
    /// Places 13 byte item on top of the stack
    Push13,
    /// Places 14 byte item on top of the stack
    Push14,
    /// Places 15 byte item on top of the stack
    Push15,
    /// Places 16 byte item on top of the stack
    Push16,
    /// Places 17 byte item on top of the stack
    Push17,
    /// Places 18 byte item on top of the stack
    Push18,
    /// Places 19 byte item on top of the stack
    Push19,
    /// Places 20 byte item on top of the stack
    Push20,
    /// Places 21 byte item on top of the stack
    Push21,
    /// Places 22 byte item on top of the stack
    Push22,
    /// Places 23 byte item on top of the stack
    Push23,
    /// Places 24 byte item on top of the stack
    Push24,
    /// Places 25 byte item on top of the stack
    Push25,
    /// Places 26 byte item on top of the stack
    Push26,
    /// Places 27 byte item on top of the stack
    Push27,
    /// Places 28 byte item on top of the stack
    Push28,
    /// Places 29 byte item on top of the stack
    Push29,
    /// Places 30 byte item on top of the stack
    Push30,
    /// Places 31 byte item on top of the stack
    Push31,
    /// Places 32 byte item on top of the stack
    Push32,
    /// Duplicates the first stack item
    Dup1,
    /// Duplicates the 2nd stack item
    Dup2,
    /// Duplicates the 3rd stack item
    Dup3,
    /// Duplicates the 4th stack item
    Dup4,
    /// Duplicates the 5th stack item
    Dup5,
    /// Duplicates the 6th stack item
    Dup6,
    /// Duplicates the 7th stack item
    Dup7,
    /// Duplicates the 8th stack item
    Dup8,
    /// Duplicates the 9th stack item
    Dup9,
    /// Duplicates the 10th stack item
    Dup10,
    /// Duplicates the 11th stack item
    Dup11,
    /// Duplicates the 12th stack item
    Dup12,
    /// Duplicates the 13th stack item
    Dup13,
    /// Duplicates the 14th stack item
    Dup14,
    /// Duplicates the 15th stack item
    Dup15,
    /// Duplicates the 16th stack item
    Dup16,
    /// Exchange the top two stack items
    Swap1,
    /// Exchange the first and third stack items
    Swap2,
    /// Exchange the first and fourth stack items
    Swap3,
    /// Exchange the first and fifth stack items
    Swap4,
    /// Exchange the first and sixth stack items
    Swap5,
    /// Exchange the first and seventh stack items
    Swap6,
    /// Exchange the first and eighth stack items
    Swap7,
    /// Exchange the first and ninth stack items
    Swap8,
    /// Exchange the first and tenth stack items
    Swap9,
    /// Exchange the first and eleventh stack items
    Swap10,
    /// Exchange the first and twelfth stack items
    Swap11,
    /// Exchange the first and thirteenth stack items
    Swap12,
    /// Exchange the first and fourteenth stack items
    Swap13,
    /// Exchange the first and fifteenth stack items
    Swap14,
    /// Exchange the first and sixteenth stack items
    Swap15,
    /// Exchange the first and seventeenth stack items
    Swap16,
    /// Append Log Record with no Topics
    Log0,
    /// Append Log Record with 1 Topic
    Log1,
    /// Append Log Record with 2 Topics
    Log2,
    /// Append Log Record with 3 Topics
    Log3,
    /// Append Log Record with 4 Topics
    Log4,
    /// Create a new account with associated code
    Create,
    /// Message-call into an account
    Call,
    /// Message-call into this account with an alternative accounts code
    Callcode,
    /// Halt execution, returning output data
    Return,
    /// Message-call into this account with an alternative accounts code, persisting the sender and
    /// value
    Delegatecall,
    /// Create a new account with associated code
    Create2,
    /// Static Message-call into an account
    Staticcall,
    /// Halt execution, reverting state changes, but returning data and remaining gas
    Revert,
    /// Invalid Instruction
    Invalid,
    /// Halt Execution and Register Account for later deletion
    Selfdestruct,
}

impl Opcode {
    /// Translates an Opcode into a string
    pub fn string(&self) -> String {
        let opcode_str = match self {
            Opcode::Stop => "00",
            Opcode::Add => "01",
            Opcode::Mul => "02",
            Opcode::Sub => "03",
            Opcode::Div => "04",
            Opcode::Sdiv => "05",
            Opcode::Mod => "06",
            Opcode::Smod => "07",
            Opcode::Addmod => "08",
            Opcode::Mulmod => "09",
            Opcode::Exp => "0a",
            Opcode::Signextend => "0b",
            Opcode::Lt => "10",
            Opcode::Gt => "11",
            Opcode::Slt => "12",
            Opcode::Sgt => "13",
            Opcode::Eq => "14",
            Opcode::Iszero => "15",
            Opcode::And => "16",
            Opcode::Or => "17",
            Opcode::Xor => "18",
            Opcode::Not => "19",
            Opcode::Byte => "1a",
            Opcode::Shl => "1b",
            Opcode::Shr => "1c",
            Opcode::Sar => "1d",
            Opcode::Sha3 => "20",
            // Opcode::Keccak => "20",
            Opcode::Address => "30",
            Opcode::Balance => "31",
            Opcode::Origin => "32",
            Opcode::Caller => "33",
            Opcode::Callvalue => "34",
            Opcode::Calldataload => "35",
            Opcode::Calldatasize => "36",
            Opcode::Calldatacopy => "37",
            Opcode::Codesize => "38",
            Opcode::Codecopy => "39",
            Opcode::Gasprice => "3a",
            Opcode::Extcodesize => "3b",
            Opcode::Extcodecopy => "3c",
            Opcode::Returndatasize => "3d",
            Opcode::Returndatacopy => "3e",
            Opcode::Blockhash => "40",
            Opcode::Coinbase => "41",
            Opcode::Timestamp => "42",
            Opcode::Number => "43",
            Opcode::Difficulty => "44",
            Opcode::Gaslimit => "45",
            Opcode::Chainid => "46",
            Opcode::Selfbalance => "47",
            Opcode::Basefee => "48",
            Opcode::Pop => "50",
            Opcode::Mload => "51",
            Opcode::Mstore => "52",
            Opcode::Mstore8 => "53",
            Opcode::Sload => "54",
            Opcode::Sstore => "55",
            Opcode::Jump => "56",
            Opcode::Jumpi => "57",
            Opcode::Pc => "58",
            Opcode::Msize => "59",
            Opcode::Gas => "5a",
            Opcode::Jumpdest => "5b",
            Opcode::Push1 => "60",
            Opcode::Push2 => "61",
            Opcode::Push3 => "62",
            Opcode::Push4 => "63",
            Opcode::Push5 => "64",
            Opcode::Push6 => "65",
            Opcode::Push7 => "66",
            Opcode::Push8 => "67",
            Opcode::Push9 => "68",
            Opcode::Push10 => "69",
            Opcode::Push11 => "6a",
            Opcode::Push12 => "6b",
            Opcode::Push13 => "6c",
            Opcode::Push14 => "6d",
            Opcode::Push15 => "6e",
            Opcode::Push16 => "6f",
            Opcode::Push17 => "70",
            Opcode::Push18 => "71",
            Opcode::Push19 => "72",
            Opcode::Push20 => "73",
            Opcode::Push21 => "74",
            Opcode::Push22 => "75",
            Opcode::Push23 => "76",
            Opcode::Push24 => "77",
            Opcode::Push25 => "78",
            Opcode::Push26 => "79",
            Opcode::Push27 => "7a",
            Opcode::Push28 => "7b",
            Opcode::Push29 => "7c",
            Opcode::Push30 => "7d",
            Opcode::Push31 => "7e",
            Opcode::Push32 => "7f",
            Opcode::Dup1 => "80",
            Opcode::Dup2 => "81",
            Opcode::Dup3 => "82",
            Opcode::Dup4 => "83",
            Opcode::Dup5 => "84",
            Opcode::Dup6 => "85",
            Opcode::Dup7 => "86",
            Opcode::Dup8 => "87",
            Opcode::Dup9 => "88",
            Opcode::Dup10 => "89",
            Opcode::Dup11 => "8a",
            Opcode::Dup12 => "8b",
            Opcode::Dup13 => "8c",
            Opcode::Dup14 => "8d",
            Opcode::Dup15 => "8e",
            Opcode::Dup16 => "8f",
            Opcode::Swap1 => "90",
            Opcode::Swap2 => "91",
            Opcode::Swap3 => "92",
            Opcode::Swap4 => "93",
            Opcode::Swap5 => "94",
            Opcode::Swap6 => "95",
            Opcode::Swap7 => "96",
            Opcode::Swap8 => "97",
            Opcode::Swap9 => "98",
            Opcode::Swap10 => "99",
            Opcode::Swap11 => "9a",
            Opcode::Swap12 => "9b",
            Opcode::Swap13 => "9c",
            Opcode::Swap14 => "9d",
            Opcode::Swap15 => "9e",
            Opcode::Swap16 => "9f",
            Opcode::Log0 => "a0",
            Opcode::Log1 => "a1",
            Opcode::Log2 => "a2",
            Opcode::Log3 => "a3",
            Opcode::Log4 => "a4",
            Opcode::Create => "f0",
            Opcode::Call => "f1",
            Opcode::Callcode => "f2",
            Opcode::Return => "f3",
            Opcode::Delegatecall => "f4",
            Opcode::Create2 => "f5",
            Opcode::Staticcall => "fa",
            Opcode::Revert => "fd",
            Opcode::Invalid => "fe",
            Opcode::Selfdestruct => "ff",
        };
        opcode_str.to_string()
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let opcode_str = self.string();
        write!(f, "{}", opcode_str)
    }
}

impl From<Opcode> for String {
    fn from(o: Opcode) -> Self {
        o.string()
    }
}
