use chumsky::{error::Cheap, prelude::*};
use chumsky_huff::opcodes::{Opcode, OPCODES, OPCODES_MAP};

#[derive(Debug, Clone)]
enum Expr {
    HexLiteral(String),
    Opcode(Opcode),
}

// Create a token mapping of keyword to opcode

fn parser() -> impl Parser<char, Vec<Expr>, Error = Simple<char>> {
    let valid_opcode = text::ident()
        .map(|ident: String| {
            OPCODES_MAP
                .get(&ident)
                .map(|opcode| Expr::Opcode(opcode.clone()))
                // TODO: this line came from copilot im not to confident in it
                .unwrap_or_else(|| Expr::HexLiteral(ident))
        })
        .padded()
        .labelled("opcode");

    // let atom = valid_opcode.or(hex_literal);
    let hex_literal = lex_hex_number();

    valid_opcode.or(hex_literal.padded()).repeated()
}

fn lex_hex_number() -> impl Parser<char, Expr, Error = Simple<char>> {
    let hex = just('0')
        .chain(just('x'))
        .chain::<char, _, _>(
            filter::<_, _, Simple<char>>(|c: &char| c.is_ascii_hexdigit()).repeated(),
        )
        .collect::<String>()
        .from_str()
        .unwrapped()
        .labelled("hex_literal")
        .map(Expr::HexLiteral);

    hex
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file_path).unwrap();

    println!("{:?}", parser().parse_recovery(src));
}
