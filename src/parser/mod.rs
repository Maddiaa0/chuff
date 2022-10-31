use chumsky::prelude::*;
use std::hash::Hash;

use crate::{
    lexer::token::{Literal, Token},
    span::Spanned,
    utils::opcodes::Opcode,
};

pub fn parser() -> impl Parser<Token, Vec<Spanned<Statement>>, Error = Simple<Token>> {
    Statement::parser()
        .repeated()
        .at_least(1)
        .then_ignore(end())
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Statement {
    Error,
    FileInclude {
        path: String,
    },
    ConstantDefinition {
        name: String,
        value: String,
    },

    // TODO fill this out with the required info
    MacroDefinition {
        name: String,
        takes: u32,
        returns: u32,
        body: Vec<Spanned<MacroBody>>,
    },
}

// TOOD: include args
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MacroBody {
    Opcode(Opcode),

    // TODO give each of these names and args
    MacroInvocation(String),
    ArgsInvocation(String),
    BuiltinInvocation(String),
    JumpLabel(String),
    JumpLabelDest(String),
    HexLiteral(Literal),
}

impl Statement {
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let include_parser = Self::parse_include();
        let macro_parser = Self::parse_macro();

        include_parser.or(macro_parser)
    }

    /// Parsers to extract nested information from the tokens
    fn string_parser() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::Str(str) => str }.labelled("string")
    }

    fn ident_parser() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::Ident(str) => str}.labelled("identifier")
    }

    // Parse high level functions
    fn parse_include() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        just(Token::Include)
            .ignore_then(Self::string_parser())
            .map_with_span(|str: String, span| (Self::FileInclude { path: str }, span))
    }

    fn parse_macro() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let macro_body = Self::parse_macro_body();

        just(Token::Define)
            .ignore_then(just(Token::Macro))
            .ignore_then(Self::ident_parser())
            .then_ignore(just(Token::OpenParen))
            // TODO: Parse the arguments
            .then_ignore(just(Token::CloseParen))
            .then_ignore(just(Token::Assign))
            // TODO: handle takes / returns            .then_ignore(just(Token::OpenBrace))
            .then_ignore(just(Token::OpenBrace))
            // TODO: remove newlines altogether?
            .then_ignore(just(Token::Newline).repeated().or_not())
            .then(macro_body)
            .then_ignore(just(Token::Newline).repeated().or_not())
            .then_ignore(just(Token::CloseBrace))
            // TODO: recover with open and close delimiters
            .map_with_span(|(name, body), span| {
                (
                    Self::MacroDefinition {
                        name,
                        takes: 0,
                        returns: 0,
                        body,
                    },
                    span,
                )
            })
    }

    fn parse_macro_body(
    ) -> impl Parser<Token, Vec<Spanned<MacroBody>>, Error = Simple<Token>> + Clone {
        let opcode = Self::parse_opcode();
        let macro_invocation = Self::parse_macro_invocation();
        let builtin_invocation = Self::parse_builtin_invocation();
        let jump_label = Self::parse_jump_label();
        let arg_invocation = Self::parse_arg_invocation();
        let hex_literal = Self::parse_hex_literal();

        opcode
            .map_with_span(|tok, span| (MacroBody::Opcode(tok), span))
            .or(macro_invocation)
            .or(hex_literal)
            .or(arg_invocation)
            .or(builtin_invocation)
            .or(jump_label)
            .repeated()
    }

    fn parse_opcode() -> impl Parser<Token, Opcode, Error = Simple<Token>> + Clone {
        select! {Token::Opcode(opcode) => opcode}.labelled("opcode")
    }

    fn parse_builtin_ident() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::BuiltinFunction(name) => name}.labelled("builtin")
    }

    fn extract_literal() -> impl Parser<Token, Literal, Error = Simple<Token>> + Clone {
        select! { Token::Literal(lit) => lit }.labelled("hex_literal")
    }

    fn parse_hex_literal() -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone
    {
        let get_literal = Self::extract_literal();

        get_literal.map_with_span(|lit, span| (MacroBody::HexLiteral(lit), span))
    }

    /// Parse Jump Label
    ///
    /// Parses jump labels in the pattern (ident, Option<:>). If the option resolves to have a value
    /// then is it determined that this is a jump location.
    fn parse_jump_label() -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone {
        let ident = Self::ident_parser();

        ident
            .then(just(Token::Colon).or_not())
            .map_with_span(|(label, is_dest), span| {
                let tok = match is_dest {
                    Some(_) => MacroBody::JumpLabelDest(label),
                    None => MacroBody::JumpLabel(label),
                };
                (tok, span)
            })
    }

    fn parse_arg_invocation(
    ) -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone {
        let ident = Self::ident_parser();

        just(Token::LeftAngle)
            .ignore_then(ident)
            .then_ignore(just(Token::RightAngle))
            .map_with_span(|arg, span| (MacroBody::ArgsInvocation(arg), span))
    }

    fn parse_builtin_invocation(
    ) -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone {
        let builtin_ident = Self::parse_builtin_ident();

        builtin_ident
            .then_ignore(just(Token::OpenParen))
            // TODO: parse with args
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|builtin, span| (MacroBody::BuiltinInvocation(builtin), span))
    }

    fn parse_macro_invocation(
    ) -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone {
        let ident = Self::ident_parser();

        ident
            .then_ignore(just(Token::OpenParen))
            // TODO: parse with args
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|ide, span| (MacroBody::MacroInvocation(ide), span))
    }
}
