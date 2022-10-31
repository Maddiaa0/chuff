use crate::parser::token::Token;
use chumsky::{prelude::*, text::Character};

pub fn key(c: String) -> impl Parser<char, (), Error = Simple<char>> + Clone {
    text::keyword(c).padded()
}

pub fn ident() -> impl Parser<char, <char as Character>::Collection, Error = Simple<char>> {
    text::ident().padded()
}

pub fn equals() -> impl Parser<char, char, Error = Simple<char>> {
    just('=').padded()
}

pub fn parse_define() -> impl Parser<char, (), Error = Simple<char>> {
    let key = |c| text::keyword(c).padded();

    just('#').then(key("define")).to(()).labelled("define")
}

/// parsees non-newline whitespace, and return nothing if successful.
pub fn parse_non_newline_whitespace() -> impl Parser<char, (), Error = Simple<char>> + Clone {
    // See https://doc.rust-lang.org/reference/whitespace.html
    one_of("\t ").to(()).labelled("whitespace")
}

pub fn parse_newline_and_comments() -> impl Parser<char, Token, Error = Simple<char>> + Clone {
    let other_whitespace = parse_non_newline_whitespace();

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
