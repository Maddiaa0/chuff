use crate::utils::{
    abi::{Constructor, Error, Event, Function},
    builtins::BuiltinFunctionKind,
    opcodes::Opcode,
};

#[derive(Debug, Clone)]
pub enum Token {
    // Literals
    /// Hex literal Functionrepresents 256 bit value
    HexLiteral(String),

    /// An opcode represents a valid evm opcode
    Opcode(Opcode),

    /// Represents a Jump Label
    JumpLabel(String),

    /// Represents a builtin function
    BuiltinFunctionKind(BuiltinFunctionKind),

    /// Represents a free storage pointer keyword
    FreeStoragePointer,

    // An ABI function definition
    AbiFunction(Function),

    /// An ABI event definition
    AbiEvent(Event),

    /// An ABI error definition
    AbiError(Error),

    /// An ABI constructor definition
    AbiConstructor(Constructor),

    /// A constant declaration
    Constant {
        name: String,
        value: Box<Token>,
    },

    MacroInvocation {
        name: String,
        args: Vec<String>,
    },

    Macro {
        name: String,
        r#type: MacroType,
        takes: u32,
        returns: u32,
        args: Vec<String>,
        body: Vec<Token>,
    },

    Newline,

    Error,
}

pub enum ABI {
    Function,
    Event,
}

#[derive(Debug, Clone)]
pub enum MacroType {
    Function,
    Macro,
}
