//! Helper functions that appear many times at the lexing level

use chumsky::{prelude::*, text::Character};

pub fn key(c: String) -> impl Parser<char, (), Error = Simple<char>> + Clone {
    text::keyword(c).padded()
}

pub fn ident() -> impl Parser<char, <char as Character>::Collection, Error = Simple<char>> {
    text::ident().padded()
}
