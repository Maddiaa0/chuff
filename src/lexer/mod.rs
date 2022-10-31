use chumsky::prelude::*;

use crate::{
    parser::{abi, utils::key},
    utils::{
        opcodes::{Opcode, OPCODES_MAP},
        types::PrimitiveEVMType,
    },
};

/// The kind of token
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
    // Literal(Literal),
    // /// Opcode
    Opcode(Opcode),
    /// Huff label (aka PC)
    Label(String),
    // // TODO: recursive dependency resolution at the lexing level?
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

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let other_whitespace = lex_non_newline_whitespace();
    let newline = lex_newline_and_comments();

    let define = lex_define();
    let include = lex_include();
    let operators = lex_operators();
    let evm_type = lex_evm_type();
    let builtin_function = lex_builtin_function();

    let opcode_or_ident = lex_opcode();

    // Erroneous tokens, but will be lexed just incase
    // let literal = lex_literal();
    let number = lex_number();

    // Single token can be the below
    let token = define
        .or(evm_type)
        .or(include)
        .or(builtin_function)
        .or(operators)
        .or(opcode_or_ident)
        .or(number)
        .or(newline.clone());

    // Parse tokens attaching TODO: spans
    let tokens = token
        .map(|tok| tok)
        .padded_by(other_whitespace.repeated())
        .repeated();

    // newline.clone().ignore_then(
    tokens
}

pub fn lex_operators() -> impl Parser<char, Token, Error = Simple<char>> {
    just('=')
        .to(Token::Assign)
        .or(just('(').to(Token::OpenParen))
        .or(just(')').to(Token::CloseParen))
        .or(just('[').to(Token::OpenBracket))
        .or(just(']').to(Token::CloseBracket))
        .or(just('{').to(Token::OpenBrace))
        .or(just('}').to(Token::CloseBrace))
        .or(just('<').to(Token::LeftAngle))
        .or(just('>').to(Token::RightAngle))
        .or(just(',').to(Token::Comma))
        .or(just(":").to(Token::Colon))
}

pub fn lex_evm_type() -> impl Parser<char, Token, Error = Simple<char>> {
    let abi_type = lex_abi_type();

    text::keyword("bool")
        .to(PrimitiveEVMType::Bool)
        .or(text::keyword("string").to(PrimitiveEVMType::String))
        .or(text::keyword("address").to(PrimitiveEVMType::Address))
        .or(abi_type)
        .map(|t: PrimitiveEVMType| Token::PrimitiveType(t))
}

pub fn lex_builtin_function() -> impl Parser<char, Token, Error = Simple<char>> {
    just('_')
        .ignore_then(just('_'))
        .ignore_then(text::ident())
        .map(|ident| Token::BuiltinFunction(ident))
}

pub fn lex_abi_type() -> impl Parser<char, PrimitiveEVMType, Error = Simple<char>> {
    let uint = lex_uint();
    let bytes = lex_bytes();
    let int = lex_int();

    uint.or(bytes).or(int)
}
pub fn lex_bytes() -> impl Parser<char, PrimitiveEVMType, Error = Simple<char>> {
    just('b')
        .ignore_then(just('y'))
        .ignore_then(just('t'))
        .ignore_then(just('e'))
        .ignore_then(just('s'))
        .ignore_then(text::digits(10).or_not())
        .map(|digits: Option<String>| match digits {
            Some(x) => PrimitiveEVMType::Bytes(x.parse().unwrap()),
            None => PrimitiveEVMType::DynBytes,
        })
}

pub fn lex_uint() -> impl Parser<char, PrimitiveEVMType, Error = Simple<char>> {
    just('u')
        .ignore_then(just('i'))
        .ignore_then(just('n'))
        .ignore_then(just('t'))
        .ignore_then(text::digits(10))
        .map(|digits: String| PrimitiveEVMType::Uint(digits.parse().unwrap()))
}

pub fn lex_int() -> impl Parser<char, PrimitiveEVMType, Error = Simple<char>> {
    just('i')
        .ignore_then(just('n'))
        .ignore_then(just('t'))
        .ignore_then(text::digits(10))
        .map(|digits: String| PrimitiveEVMType::Int(digits.parse().unwrap()))
}

/// Lex Number
///
/// Lex number has lower precedence than lex literal, as it is used to parse them
/// validly, but show a warning to the user
pub fn lex_number() -> impl Parser<char, Token, Error = Simple<char>> {
    text::digits(16).map(|n: String| Token::Num(n.parse().unwrap_or(0)))
}

pub fn lex_opcode() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident()
        .map(|ident: String| {
            let is_opcode = OPCODES_MAP.get(&ident);

            match is_opcode {
                Some(opcode) => Token::Opcode(opcode.clone()),
                None => match ident.as_str() {
                    "macro" => Token::Macro,
                    "calldata" => Token::Calldata,
                    "memory" => Token::Memory,
                    "storage" => Token::Storage,
                    "constant" => Token::Constant,
                    "fn" => Token::Fn,
                    "function" => Token::Function,
                    "event" => Token::Event,
                    "error" => Token::Error,
                    "takes" => Token::Takes,
                    "returns" => Token::Returns,
                    "jumptable" => Token::JumpTable,
                    "jumptablepacked" => Token::JumpTablePacked,
                    "table" => Token::CodeTable,

                    _ => Token::Ident(ident),
                },
            }
        })
        // TODO: this line came from copilot im not to confident in it
        // .unwrap_or_else(|| Token::Unknown(ident))
        .padded()
        .labelled("opcode")
}

pub fn lex_define() -> impl Parser<char, Token, Error = Simple<char>> {
    just('#')
        .then(key("define".to_string()))
        .to(Token::Define)
        .labelled("define")
}

pub fn lex_include() -> impl Parser<char, Token, Error = Simple<char>> {
    just('#')
        .then(key("include".to_string()))
        .to(Token::Include)
        .labelled("include")
}
/// Lexes newlines, handling both CRLF and LF. Multiple consecutive newlines are
/// collapsed into one for the sake of simpler parsing further down the compilation
/// chain. (though of course, do NOT assume that newline tokens won't be followed by other
/// newline tokens!)
///
/// This function also handles tossing out comments. Since comments can only occur
/// at either the end of a line or completely on their own line, they should be
/// collapsed down into newline tokens.
fn lex_newline_and_comments() -> impl Parser<char, Token, Error = Simple<char>> + Clone {
    let other_whitespace = lex_non_newline_whitespace();

    let comment = just("//")
        .then(take_until(just('\n')))
        .padded_by(other_whitespace.repeated())
        .to(())
        .labelled("comment");

    text::newline()
        .or(comment)
        .repeated()
        .at_least(1)
        .to(Token::Newline)
        .labelled("newline")
}

/// Lexes non-newline whitespace, and return nothing if successful.
fn lex_non_newline_whitespace() -> impl Parser<char, (), Error = Simple<char>> + Clone {
    // See https://doc.rust-lang.org/reference/whitespace.html
    one_of("\t ").to(()).labelled("whitespace")
}
