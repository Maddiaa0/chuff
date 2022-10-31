use phf::phf_map;
use std::fmt;

/// All the EVM opcodes as a static array
/// They are arranged in a particular order such that all the opcodes that have common
/// prefixes are ordered by decreasing length to avoid mismatch when parseing.
/// Example : [origin, or] or [push32, ..., push3]
pub const OPCODES: [&str; 146] = [
    "lt",
    "gt",
    "slt",
    "sgt",
    "eq",
    "iszero",
    "and",
    "origin",
    "or",
    "xor",
    "not",
    "sha3",
    "address",
    "balance",
    "caller",
    "callvalue",
    "calldataload",
    "calldatasize",
    "calldatacopy",
    "codesize",
    "codecopy",
    "basefee",
    "blockhash",
    "coinbase",
    "timestamp",
    "number",
    "difficulty",
    "prevrandao",
    "gaslimit",
    "chainid",
    "selfbalance",
    "pop",
    "mload",
    "mstore8",
    "mstore",
    "sload",
    "sstore",
    "jumpdest",
    "jumpi",
    "jump",
    "pc",
    "msize",
    "stop",
    "addmod",
    "add",
    "mulmod",
    "mul",
    "sub",
    "div",
    "sdiv",
    "mod",
    "smod",
    "exp",
    "signextend",
    "byte",
    "shl",
    "shr",
    "sar",
    "gasprice",
    "extcodesize",
    "extcodecopy",
    "returndatasize",
    "returndatacopy",
    "extcodehash",
    "gas",
    "log0",
    "log1",
    "log2",
    "log3",
    "log4",
    "tload",
    "tstore",
    "create2",
    "create",
    "callcode",
    "call",
    "return",
    "delegatecall",
    "staticcall",
    "revert",
    "invalid",
    "selfdestruct",
    "push32",
    "push31",
    "push30",
    "push29",
    "push28",
    "push27",
    "push26",
    "push25",
    "push24",
    "push23",
    "push22",
    "push21",
    "push20",
    "push19",
    "push18",
    "push17",
    "push16",
    "push15",
    "push14",
    "push13",
    "push12",
    "push11",
    "push10",
    "push9",
    "push8",
    "push7",
    "push6",
    "push5",
    "push4",
    "push3",
    "push2",
    "push1",
    "swap16",
    "swap15",
    "swap14",
    "swap13",
    "swap12",
    "swap11",
    "swap10",
    "swap9",
    "swap8",
    "swap7",
    "swap6",
    "swap5",
    "swap4",
    "swap3",
    "swap2",
    "swap1",
    "dup16",
    "dup15",
    "dup14",
    "dup13",
    "dup12",
    "dup11",
    "dup10",
    "dup9",
    "dup8",
    "dup7",
    "dup6",
    "dup5",
    "dup4",
    "dup3",
    "dup2",
    "dup1",
];

/// Hashmap of all the EVM opcodes
pub static OPCODES_MAP: phf::Map<&'static str, Opcode> = phf_map! {
    "lt" => Opcode::Lt,
    "gt" => Opcode::Gt,
    "slt" => Opcode::Slt,
    "sgt" => Opcode::Sgt,
    "eq" => Opcode::Eq,
    "iszero" => Opcode::Iszero,
    "and" => Opcode::And,
    "or" => Opcode::Or,
    "xor" => Opcode::Xor,
    "not" => Opcode::Not,
    "sha3" => Opcode::Sha3,
    "address" => Opcode::Address,
    "balance" => Opcode::Balance,
    "origin" => Opcode::Origin,
    "caller" => Opcode::Caller,
    "callvalue" => Opcode::Callvalue,
    "calldataload" => Opcode::Calldataload,
    "calldatasize" => Opcode::Calldatasize,
    "calldatacopy" => Opcode::Calldatacopy,
    "codesize" => Opcode::Codesize,
    "codecopy" => Opcode::Codecopy,
    "basefee" => Opcode::Basefee,
    "blockhash" => Opcode::Blockhash,
    "coinbase" => Opcode::Coinbase,
    "timestamp" => Opcode::Timestamp,
    "number" => Opcode::Number,
    "difficulty" => Opcode::Difficulty,
    "prevrandao" => Opcode::Prevrandao,
    "gaslimit" => Opcode::Gaslimit,
    "chainid" => Opcode::Chainid,
    "selfbalance" => Opcode::Selfbalance,
    "pop" => Opcode::Pop,
    "mload" => Opcode::Mload,
    "mstore" => Opcode::Mstore,
    "mstore8" => Opcode::Mstore8,
    "sload" => Opcode::Sload,
    "sstore" => Opcode::Sstore,
    "jump" => Opcode::Jump,
    "jumpi" => Opcode::Jumpi,
    "pc" => Opcode::Pc,
    "msize" => Opcode::Msize,
    "push1" => Opcode::Push1,
    "push2" => Opcode::Push2,
    "push3" => Opcode::Push3,
    "push4" => Opcode::Push4,
    "push5" => Opcode::Push5,
    "push6" => Opcode::Push6,
    "push7" => Opcode::Push7,
    "push8" => Opcode::Push8,
    "push9" => Opcode::Push9,
    "push10" => Opcode::Push10,
    "push17" => Opcode::Push17,
    "push18" => Opcode::Push18,
    "push19" => Opcode::Push19,
    "push20" => Opcode::Push20,
    "push21" => Opcode::Push21,
    "push22" => Opcode::Push22,
    "push23" => Opcode::Push23,
    "push24" => Opcode::Push24,
    "push25" => Opcode::Push25,
    "push26" => Opcode::Push26,
    "dup1" => Opcode::Dup1,
    "dup2" => Opcode::Dup2,
    "dup3" => Opcode::Dup3,
    "dup4" => Opcode::Dup4,
    "dup5" => Opcode::Dup5,
    "dup6" => Opcode::Dup6,
    "dup7" => Opcode::Dup7,
    "dup8" => Opcode::Dup8,
    "dup9" => Opcode::Dup9,
    "dup10" => Opcode::Dup10,
    "swap1" => Opcode::Swap1,
    "swap2" => Opcode::Swap2,
    "swap3" => Opcode::Swap3,
    "swap4" => Opcode::Swap4,
    "swap5" => Opcode::Swap5,
    "swap6" => Opcode::Swap6,
    "swap7" => Opcode::Swap7,
    "swap8" => Opcode::Swap8,
    "swap9" => Opcode::Swap9,
    "swap10" => Opcode::Swap10,
    "stop" => Opcode::Stop,
    "add" => Opcode::Add,
    "mul" => Opcode::Mul,
    "sub" => Opcode::Sub,
    "div" => Opcode::Div,
    "sdiv" => Opcode::Sdiv,
    "mod" => Opcode::Mod,
    "smod" => Opcode::Smod,
    "addmod" => Opcode::Addmod,
    "mulmod" => Opcode::Mulmod,
    "exp" => Opcode::Exp,
    "signextend" => Opcode::Signextend,
    "byte" => Opcode::Byte,
    "shl" => Opcode::Shl,
    "shr" => Opcode::Shr,
    "sar" => Opcode::Sar,
    "gasprice" => Opcode::Gasprice,
    "extcodesize" => Opcode::Extcodesize,
    "extcodecopy" => Opcode::Extcodecopy,
    "returndatasize" => Opcode::Returndatasize,
    "returndatacopy" => Opcode::Returndatacopy,
    "extcodehash" => Opcode::Extcodehash,
    "gas" => Opcode::Gas,
    "jumpdest" => Opcode::Jumpdest,
    "push11" => Opcode::Push11,
    "push12" => Opcode::Push12,
    "push13" => Opcode::Push13,
    "push14" => Opcode::Push14,
    "push15" => Opcode::Push15,
    "push16" => Opcode::Push16,
    "push27" => Opcode::Push27,
    "push28" => Opcode::Push28,
    "push29" => Opcode::Push29,
    "push30" => Opcode::Push30,
    "push31" => Opcode::Push31,
    "push32" => Opcode::Push32,
    "dup11" => Opcode::Dup11,
    "dup12" => Opcode::Dup12,
    "dup13" => Opcode::Dup13,
    "dup14" => Opcode::Dup14,
    "dup15" => Opcode::Dup15,
    "dup16" => Opcode::Dup16,
    "swap11" => Opcode::Swap11,
    "swap12" => Opcode::Swap12,
    "swap13" => Opcode::Swap13,
    "swap14" => Opcode::Swap14,
    "swap15" => Opcode::Swap15,
    "swap16" => Opcode::Swap16,
    "log0" => Opcode::Log0,
    "log1" => Opcode::Log1,
    "log2" => Opcode::Log2,
    "log3" => Opcode::Log3,
    "log4" => Opcode::Log4,
    "tload" => Opcode::TLoad,
    "tstore" => Opcode::TStore,
    "create" => Opcode::Create,
    "call" => Opcode::Call,
    "callcode" => Opcode::Callcode,
    "return" => Opcode::Return,
    "delegatecall" => Opcode::Delegatecall,
    "staticcall" => Opcode::Staticcall,
    "create2" => Opcode::Create2,
    "revert" => Opcode::Revert,
    "invalid" => Opcode::Invalid,
    "selfdestruct" => Opcode::Selfdestruct
};

/// EVM Opcodes
/// References <https://evm.codes>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    /// Halts execution.
    Stop,
    /// Addition operation
    Add,
    /// Multiplication Operation
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
    /// Pseudorandomness from the Beacon Chain
    Prevrandao,
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
    /// Transaction-persistent, but storage-ephemeral variable load
    TLoad,
    /// Transaction-persistent, but storage-ephemeral variable store
    TStore,
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
    /// Get hash of an account’s code
    Extcodehash,
}
