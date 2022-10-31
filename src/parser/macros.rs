use chumsky::{prelude::*, text::TextParser};

use crate::utils::{builtins::BUILTINS_MAP, opcodes::OPCODES_MAP};

use super::{
    token::{MacroType, Token},
    utils::parse_newline_and_comments,
};

/// parse Macro
///
/// Steps:
///     1. Find `#define` keyword
///     2. the macro keyword
///     3. the macro arguments inside ( )
///     4. find equals
///     5. Find the takes() value, default to 0
///     6. Find the returns() value, default to 0
///     7. Find the macro body
///
/// TODO: right now do not allow nested macros, that will come later
pub fn parse_macro() -> impl Parser<char, Token, Error = Simple<char>> {
    // Pad keyword finders
    let ident = text::ident().padded();
    let key = |c| text::keyword(c).padded();
    let char = |c| just(c).padded();

    // Other parseers
    let macro_body = parse_macro_body();
    let macro_type = parse_macro_type();

    just('#')
        .ignore_then(key("define"))
        .ignore_then(macro_type)
        .then(ident)
        .then_ignore(char('('))
        // TODO: Parse the macro arguments
        .then_ignore(char(')'))
        .then_ignore(char('='))
        // TODO: turn takes into its own parse so the whole thing can be if or
        .then_ignore(key("takes"))
        .then_ignore(char('('))
        .then(text::digits(10).or_not())
        .then_ignore(char(')'))
        // TODO: turn returns into its own parse so the whole thing can be if or
        .then_ignore(key("returns"))
        .then_ignore(char('('))
        .then(text::digits(10).or_not())
        .then_ignore(char(')'))
        .then_ignore(char('{'))
        .then(macro_body)
        .then_ignore(char('}'))
        .map(|((((macro_type, name), takes), returns), macros)| {
            // println!("{name} {:?}", name = name, takes);
            Token::Macro {
                name: name,
                // TODO: clean up this line
                r#type: macro_type,
                takes: takes.unwrap_or(0.to_string()).parse().unwrap(),
                returns: returns.unwrap_or(0.to_string()).parse().unwrap(),
                args: vec![],
                body: macros,
            }
        })
        .labelled("macro_body")
        .padded()
}

fn parse_macro_type() -> impl Parser<char, MacroType, Error = Simple<char>> {
    let key = |c| text::keyword(c).padded();

    key("fn")
        .map(|_| MacroType::Function)
        .or(key("macro").map(|_| MacroType::Macro))
}

pub fn parse_macro_body() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let newline = parse_newline_and_comments();
    let opcode = parse_opcode_or_jump_label();
    let hex_literal = parse_hex_number();
    let macro_invocation = parse_macro_invocation();
    let builtin_fn = parse_builtin_fn();

    builtin_fn
        .or(macro_invocation)
        .or(opcode)
        .or(hex_literal)
        .or(newline.clone())
        .repeated()
}

fn parse_macro_invocation() -> impl Parser<char, Token, Error = Simple<char>> {
    let ident = text::ident();

    ident
        .then_ignore(just('('))
        // TODO: args delimited by comma
        .then_ignore(just(')'))
        .map(|name| Token::MacroInvocation { name, args: vec![] })
        .labelled("macro_invocation")
}

/// parse Builtin function invocations
fn parse_builtin_fn() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident()
        .then_ignore(just('('))
        // TODO: parse args
        .then_ignore(just(')'))
        .map(|ident: String| {
            BUILTINS_MAP
                .get(&ident)
                .map(|builtin| Token::BuiltinFunctionKind(builtin.clone()))
                // TODO: this line came from copilot im not to confident in it
                .unwrap_or_else(|| Token::JumpLabel(ident))
        })
        .padded()
        .labelled("builtin_fn_invocation")
}

pub fn parse_hex_number() -> impl Parser<char, Token, Error = Simple<char>> {
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

fn parse_opcode_or_jump_label() -> impl Parser<char, Token, Error = Simple<char>> {
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
