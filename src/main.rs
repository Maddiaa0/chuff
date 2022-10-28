use chumsky::{error::Cheap, prelude::*};
use chumsky_huff::opcodes::{Opcode, OPCODES, OPCODES_MAP};

#[derive(Debug, Clone)]
enum Token {
    HexLiteral(String),
    Opcode(Opcode),
    Newline,
    JumpLabel(String),
}

// Create a token mapping of keyword to opcode

fn parser() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let newline = lex_newline_and_comments();
    let opcode = lex_opcode_or_jump_label();
    let hex_literal = lex_hex_number();

    opcode.or(hex_literal).or(newline.clone()).repeated()
}

fn lex_hex_number() -> impl Parser<char, Token, Error = Simple<char>> {
    just('0')
        .chain(just('x'))
        .chain::<char, _, _>(
            filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_hexdigit()).repeated(),
        )
        .collect::<String>()
        .from_str()
        .unwrapped()
        .labelled("hex_literal")
        .map(Token::HexLiteral)
        .padded()
}

fn lex_opcode_or_jump_label() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident()
        .map(|ident: String| {
            OPCODES_MAP
                .get(&ident)
                .map(|opcode| Token::Opcode(opcode.clone()))
                // TODO: this line came from copilot im not to confident in it
                .unwrap_or_else(|| Token::JumpLabel(ident))
        })
        .padded()
        .labelled("opcode")
}

fn lex_newline_and_comments() -> impl Parser<char, Token, Error = Simple<char>> + Clone {
    let other_whitespace = lex_non_newline_whitespace();

    let comment = just("//")
        .then(take_until(just("\n")))
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

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file_path).unwrap();

    println!("{:?}", parser().parse_recovery(src));
}
