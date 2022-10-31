// TOOD: look in ast.rs of huff-rs and rip the types so that they can
// be identical to the ones in the huff-rs crate

use chumsky::{prelude::*, Stream};
use chumsky_huff::{
    lexer::{lexer, token::Token},
    parser::parser,
};

/// Error strategies
// skip_then_retry_with();
// skip_then_retry_until
// skip_until
// nested_delimiters

// Create a token mapping of keyword to opcode

fn main() -> Result<(), String> {
    let file_path = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file_path).unwrap();
    let src_len = src.chars().count();

    // .parse_recovery(src).
    let (tokens, lex_errors) = lexer().parse_recovery(src);
    println!("TOKENS");
    println!("{tokens:?}");

    // TODO: remove newlines from the tokens

    let (ast, parse_errs) = if let Some(tokens) = tokens {
        let clean_tokens = tokens
            .into_iter()
            .filter(|(token, _)| token.clone() != Token::Newline);
        let token_stream = Stream::from_iter(src_len..src_len + 1, clean_tokens.into_iter());
        parser().parse_recovery(token_stream)
    } else {
        (None, vec![])
    };
    println!("AST");
    println!("{ast:?}");

    println!("ERRS");
    println!("{parse_errs:?}");
    // let debug = parser().parse_recovery_verbose(src);
    // println!("{:?}", debug);

    Ok(())
}
