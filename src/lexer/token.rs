use crate::utils::{opcodes::Opcode, types::PrimitiveEVMType};

pub type Literal = [u8; 32];

/// The kind of token
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    /// EOF Token
    Eof,
    /// A Comment
    Comment(String),
    /// A newline
    Newline,
    /// Division
    /// Lexing done at the comment level due to clash
    Div,
    /// "#define" keyword
    Define,
    /// "#include" keyword
    Include,
    /// "macro" keyword
    Macro,
    /// "fn" keyword
    Fn,
    /// "test" keyword
    Test,
    /// "function" keyword
    Function,
    /// "event" keyword
    Event,
    /// "constant" keyword
    Constant,
    /// "error" keyword
    Error,
    /// "takes" keyword
    Takes,
    /// "returns" keyword
    Returns,
    /// "view" keyword
    View,
    /// "pure" keyword
    Pure,
    /// "payable" keyword
    Payable,
    /// "nonpayable" keyword
    NonPayable,
    /// "indexed" keyword
    Indexed,
    /// "FREE_STORAGE_POINTER()" keyword
    FreeStoragePointer,
    /// An Identifier
    Ident(String),
    /// Equal Sign
    Assign,
    /// An open parenthesis
    OpenParen,
    /// A close parenthesis
    CloseParen,
    /// An open bracket
    OpenBracket,
    /// A close bracket
    CloseBracket,
    /// An open brace
    OpenBrace,
    /// A close brace
    CloseBrace,
    /// A Less-Than Angle Bracket
    LeftAngle,
    /// A Greater-Than Angle Bracket
    RightAngle,
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// A comma
    Comma,
    /// A Colon
    Colon,
    /// A pound
    Pound,
    /// Number
    Num(usize),
    /// A Space
    Whitespace,
    /// A string literal
    Str(String),
    /// Hex
    Literal(Literal),
    /// Code snippet
    Code(String),
    // /// Opcode
    Opcode(Opcode),
    /// Huff label (aka PC)
    Label(String),
    // Import path
    Path(String),
    /// EVM Type
    PrimitiveType(PrimitiveEVMType),
    /// Array of EVM Types
    /// uint256[5][2][3] => ArrayType(PrimitiveEVMType::Uint(256), [5, 2, 3])
    ArrayType(PrimitiveEVMType, Vec<usize>),
    /// A Jump Table
    JumpTable,
    /// A Packed Jump Table
    JumpTablePacked,
    /// A Code Table
    CodeTable,
    /// A builtin function (__codesize, __tablesize, __tablestart)
    BuiltinFunction(String),
    /// Calldata Data Location
    Calldata,
    /// Memory Data Location
    Memory,
    /// Storage Data Location
    Storage,

    Unknown(String),
}
