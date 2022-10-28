use chumsky::prelude::*;
use chumsky_huff::opcodes::{Opcode, OPCODES, OPCODES_MAP};

#[derive(Debug, Clone)]
enum Expr {
    HexLiteral(u64),
    Opcode(Opcode),
}

// Create a token mapping of keyword to opcode

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let opcode = text::ident().map(|ident: String| {
        OPCODES_MAP
            .get(&ident)
            .map(|opcode| Expr::Opcode(opcode.clone()))
            // TODO: this line came from copilot im not to confident in it
            .unwrap_or_else(|| Expr::HexLiteral(u64::from_str_radix(&ident, 16).unwrap()))
    });

    opcode.then_ignore(end())
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file_path).unwrap();

    println!("{:?}", parser().parse_recovery(src));
}
