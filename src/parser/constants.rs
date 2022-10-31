use chumsky::{prelude::*, text::TextParser};

use super::{
    macros::parse_hex_number,
    token::Token,
    utils::{equals, ident, key, parse_define},
};

/// Constant parseer
///
/// Find constants in the program, they are defined as
/// `#define constant <name> = <value>`
/// where value can either be FREE_STORAGE_POINTER() or a hex literal
pub fn parse_constant() -> impl Parser<char, Token, Error = Simple<char>> {
    let hex_literal = parse_hex_number();
    let free_storage_pointer = parse_free_storage_pointer();
    let define = parse_define();
    let constant_name = ident;

    let valid_constant_body = hex_literal
        .or(free_storage_pointer)
        .labelled("valid_constant_body");

    define
        .ignore_then(key("constant".to_string()))
        .ignore_then(constant_name())
        .then_ignore(equals())
        .then(valid_constant_body)
        .map(|(name, value)| Token::Constant {
            name: name,
            value: Box::from(value),
        })
}

/// Free storage pointer parseer
///
/// Match against `FREE_STORAGE_POINTER()`
fn parse_free_storage_pointer() -> impl Parser<char, Token, Error = Simple<char>> {
    key("FREE".to_string())
        .then_ignore(just('_'))
        .then_ignore(key("STORAGE".to_string()))
        .then_ignore(just('_'))
        .then_ignore(key("POINTER".to_string()))
        .then_ignore(just('('))
        .then_ignore(just(')'))
        .map(|_| Token::FreeStoragePointer)
}
