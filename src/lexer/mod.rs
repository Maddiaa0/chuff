pub mod token;
pub mod utils;

use chumsky::prelude::*;
use {token::Token, utils::key};

use crate::{
    span::Spanned,
    utils::{bytes_util::str_to_bytes32, opcodes::OPCODES_MAP, types::PrimitiveEVMType},
};

pub fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
    let other_whitespace = lex_non_newline_whitespace();
    let newline = lex_newline_and_comments();

    let define = lex_define();
    let include = lex_include();
    let hex_literals = lex_literals();
    let operators = lex_operators();
    let evm_type = lex_evm_type();
    let builtin_function = lex_builtin_function();
    let string = lex_string(); // Only relevant to file imports
    let free_storage_pointer = lex_free_storage_pointer();

    let opcode_or_ident = lex_opcode();

    // Erroneous tokens, but will be lexed just incase
    let number = lex_number();

    // Single token can be the below
    let token = define
        .or(evm_type)
        .or(free_storage_pointer)
        .or(include)
        .or(string)
        .or(hex_literals)
        .or(builtin_function)
        .or(operators)
        .or(opcode_or_ident)
        .or(number)
        .or(newline.clone())
        // Skip invalid characters
        .recover_with(skip_then_retry_until([]));

    let tokens = token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(other_whitespace.repeated())
        .repeated() // make sure there's a newline at the end of input
        .chain(
            newline
                .clone()
                // if there isn't a newline at the end of input, just insert a fake newline token
                .or(end().rewind().to(Token::Newline))
                // make sure to attach a span! this might be incorrect for the fake newlines
                .map_with_span(|tok, span| (tok, span)),
        )
        .then_ignore(end());

    newline.clone().or_not().ignore_then(tokens)
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

pub fn lex_literals() -> impl Parser<char, Token, Error = Simple<char>> {
    just('0')
        .ignore_then(just('x'))
        .ignore_then(text::digits(16))
        .map(|num: String| {
            // work out when to return it as an identifier
            if num.len() < 64 {
                return Token::Literal(str_to_bytes32(&num));
            } else {
                return Token::Code(num.clone());
            }
        })
}

pub fn lex_evm_type() -> impl Parser<char, Token, Error = Simple<char>> {
    let abi_type = lex_abi_type();
    let arr = lex_array();

    text::keyword("bool")
        .to(PrimitiveEVMType::Bool)
        .or(text::keyword("string").to(PrimitiveEVMType::String))
        .or(text::keyword("address").to(PrimitiveEVMType::Address))
        .or(abi_type)
        .then(arr.or_not())
        .map(|(prim, is_arr)| match is_arr {
            Some(arr) => {
                if arr.is_empty() {
                    Token::PrimitiveType(prim)
                } else {
                    Token::ArrayType(prim, arr)
                }
            }
            None => Token::PrimitiveType(prim),
        })
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

/// Lex Array
///
pub fn lex_array() -> impl Parser<char, Vec<usize>, Error = Simple<char>> {
    just('[')
        .ignore_then(text::digits(10).or_not())
        .then_ignore(just(']'))
        .map(|num: Option<String>| match num {
            Some(x) => x.parse().unwrap(),
            None => 0,
        })
        .repeated()
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

/// Lexes a string. Currently, there is no way to escape quotes inside of strings.
fn lex_string() -> impl Parser<char, Token, Error = Simple<char>> + Clone {
    let escape = just('\\')
        .ignore_then(
            just('\\')
                .or(just('/'))
                .or(just('"'))
                .or(just('b').to('\x08'))
                .or(just('f').to('\x0C'))
                .or(just('n').to('\n'))
                .or(just('r').to('\r'))
                .or(just('t').to('\t'))
                .or(just('u').ignore_then(
                    // unicode UTF-32 escapes
                    filter(|c: &char| c.is_ascii_hexdigit())
                        .repeated()
                        .exactly(4)
                        .collect::<String>()
                        .validate(|digits, span, emit| {
                            char::from_u32(u32::from_str_radix(&digits, 16).unwrap())
                                .unwrap_or_else(|| {
                                    emit(Simple::custom(span, "Invalid unicode character"));
                                    '\u{FFFD}' // unicode replacement character
                                })
                        }),
                )),
        )
        .labelled("string escape character");

    just('"')
        .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Str)
        .labelled("string")
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
                    "codetable" => Token::CodeTable,
                    "jumptable" => Token::JumpTable,
                    "jumptablepacked" => Token::JumpTablePacked,

                    // TODO: do we only want these to lex as these tokens in a specific context?
                    "pure" => Token::Pure,
                    "payable" => Token::Payable,
                    "nonpayable" => Token::NonPayable,
                    "view" => Token::View,
                    "returns" => Token::Returns,
                    "indexed" => Token::Indexed,

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

pub fn lex_free_storage_pointer() -> impl Parser<char, Token, Error = Simple<char>> {
    key("FREE_STORAGE_POINTER".to_string()).to(Token::FreeStoragePointer)
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
