use std::fmt::Display;

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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Eof => write!(f, "EOF"),
            Token::Comment(c) => write!(f, "Comment({})", c),
            Token::Newline => write!(f, "Newline"),
            Token::Div => write!(f, "Div"),
            Token::Define => write!(f, "Define"),
            Token::Include => write!(f, "Include"),
            Token::Macro => write!(f, "Macro"),
            Token::Fn => write!(f, "Fn"),
            Token::Test => write!(f, "Test"),
            Token::Function => write!(f, "Function"),
            Token::Event => write!(f, "Event"),
            Token::Constant => write!(f, "Constant"),
            Token::Error => write!(f, "Error"),
            Token::Takes => write!(f, "Takes"),
            Token::Returns => write!(f, "Returns"),
            Token::View => write!(f, "View"),
            Token::Pure => write!(f, "Pure"),
            Token::Payable => write!(f, "Payable"),
            Token::NonPayable => write!(f, "NonPayable"),
            Token::Indexed => write!(f, "Indexed"),
            Token::FreeStoragePointer => write!(f, "FreeStoragePointer"),
            Token::Ident(i) => write!(f, "Ident({})", i),
            Token::Assign => write!(f, "="),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBracket => write!(f, "["),
            Token::CloseBracket => write!(f, "]"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::LeftAngle => write!(f, "<"),
            Token::RightAngle => write!(f, ">"),
            Token::Add => write!(f, "Add"),
            Token::Sub => write!(f, "Sub"),
            Token::Mul => write!(f, "Mul"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ";"),
            Token::Pound => write!(f, "#"),
            Token::Num(n) => write!(f, "Num({})", n),
            Token::Whitespace => write!(f, "Whitespace"),
            Token::Str(s) => write!(f, "Str({})", s),
            Token::Literal(l) => write!(
                f,
                "Literal({})",
                l.iter()
                    .fold(String::new(), |acc, x| acc + &format!("{:02x}", x))
            ),
            Token::Code(c) => write!(f, "Code({})", c),
            Token::Opcode(o) => write!(f, "Opcode({})", o),
            Token::Label(l) => write!(f, "Label({})", l),
            Token::Path(p) => write!(f, "Path({})", p),
            Token::PrimitiveType(t) => write!(f, "PrimitiveType({})", t),
            Token::ArrayType(t, d) => write!(f, "ArrayType({}, {:?})", t, d),
            Token::JumpTable => write!(f, "JumpTable"),
            Token::JumpTablePacked => write!(f, "JumpTablePacked"),
            Token::CodeTable => write!(f, "CodeTable"),
            Token::BuiltinFunction(b) => write!(f, "BuiltinFunction({})", b),
            Token::Calldata => write!(f, "Calldata"),
            Token::Memory => write!(f, "Memory"),
            Token::Storage => write!(f, "Storage"),
            Token::Unknown(u) => write!(f, "Unknown({})", u),
        }
    }
}
