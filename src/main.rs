// TOOD: look in ast.rs of huff-rs and rip the types so that they can
// be identical to the ones in the huff-rs crate

use chumsky::prelude::*;
use chumsky_huff::{
    parser::{
        constants::lex_constant, macros::lex_macro, token::Token, utils::lex_newline_and_comments,
    },
    utils::{
        abi::{Constructor, Error, Event, Function},
        builtins::{BuiltinFunctionKind, BUILTINS_MAP},
        opcodes::{Opcode, OPCODES_MAP},
    },
};

/// Error strategies
// skip_then_retry_with();
// skip_then_retry_until
// skip_until
// nested_delimiters

// Create a token mapping of keyword to opcode

fn parser() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let program = lex_program();

    program.then_ignore(end())
}

fn lex_program() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let macro_lexer = lex_macro();
    let newline = lex_newline_and_comments();
    let constant = lex_constant();

    macro_lexer
        .or(newline)
        .or(constant)
        // Naive strategy ignores unexpected definitions
        .recover_with(skip_then_retry_until(['#']))
        .repeated()
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file_path).unwrap();

    println!("{:?}", parser().parse_recovery(src));
}
