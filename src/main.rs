use chumsky::{
    error::Cheap,
    prelude::*,
    text::{whitespace, TextParser},
};
use chumsky_huff::opcodes::{Opcode, OPCODES, OPCODES_MAP};

#[derive(Debug, Clone)]
enum Token {
    // Literals
    /// Hex literal represents 256 bit value
    HexLiteral(String),

    /// An opcode represents a valid evm opcode
    Opcode(Opcode),

    /// Represents a Jump Label
    JumpLabel(String),

    /// Represents a free storage pointer keyword
    FreeStoragePointer,

    /// A constant declaration
    Constant {
        name: String,
        value: Box<Token>,
    },

    MacroInvocation {
        name: String,
        args: Vec<String>,
    },

    Macro {
        name: String,
        takes: u32,
        returns: u32,
        args: Vec<String>,
        body: Vec<Token>,
    },

    Newline,

    Error,
}

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
        // TODO: recover to the next # for a macro definition
        // .recover_with(strategy)
        .repeated()
}

/// Lex Macro
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
fn lex_macro() -> impl Parser<char, Token, Error = Simple<char>> {
    // Pad keyword finders
    let ident = text::ident().padded();
    let key = |c| text::keyword(c).padded();
    let char = |c| just(c).padded();

    // Other lexers
    let macro_body = lex_macro_body();

    just('#')
        .ignore_then(key("define"))
        .ignore_then(key("macro"))
        .ignore_then(ident)
        .then_ignore(char('='))
        // TODO: turn takes into its own lex so the whole thing can be if or
        .then_ignore(key("takes"))
        .then_ignore(char('('))
        .then(text::digits(10).or_not())
        .then_ignore(char(')'))
        // TODO: turn returns into its own lex so the whole thing can be if or
        .then_ignore(key("returns"))
        .then_ignore(char('('))
        .then(text::digits(10).or_not())
        .then_ignore(char(')'))
        .then_ignore(char('{'))
        .then(macro_body)
        .then_ignore(char('}'))
        .map(|(((name, takes), returns), macros)| {
            // println!("{name} {:?}", name = name, takes);
            Token::Macro {
                name: name,
                // TODO: clean up this line
                takes: takes.unwrap_or(0.to_string()).parse().unwrap(),
                returns: returns.unwrap_or(0.to_string()).parse().unwrap(),
                args: vec![],
                body: macros,
            }
        })
        .labelled("macro_body")
        .padded()
}

/// Constant Lexer
///
/// Find constants in the program, they are defined as
/// `#define constant <name> = <value>`
/// where value can either be FREE_STORAGE_POINTER() or a hex literal
fn lex_constant() -> impl Parser<char, Token, Error = Simple<char>> {
    let hex_literal = lex_hex_number();
    let free_storage_pointer = lex_free_storage_pointer();
    let define = lex_define();
    let key = |c| text::keyword(c).padded();
    let ident = text::ident().padded();
    let constant_name = ident;
    let equals = just('=').padded();

    let valid_constant_body = hex_literal
        .or(free_storage_pointer)
        .labelled("valid_constant_body");

    define
        .ignore_then(key("constant"))
        .ignore_then(constant_name)
        .then_ignore(equals)
        .then(valid_constant_body)
        .map(|(name, value)| Token::Constant {
            name: name,
            value: Box::from(value),
        })
}

/// Free storage pointer lexer
///
/// Match against `FREE_STORAGE_POINTER()`
fn lex_free_storage_pointer() -> impl Parser<char, Token, Error = Simple<char>> {
    // TODO: prevent repetition of these building blocks
    let key = |c| text::keyword(c).padded();

    key("FREE")
        .then_ignore(just('_'))
        .then_ignore(key("STORAGE"))
        .then_ignore(just('_'))
        .then_ignore(key("POINTER"))
        .then_ignore(just('('))
        .then_ignore(just(')'))
        .map(|_| Token::FreeStoragePointer)
}

fn lex_define() -> impl Parser<char, (), Error = Simple<char>> {
    let key = |c| text::keyword(c).padded();

    just('#').then(key("define")).to(()).labelled("define")
}

fn lex_macro_body() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let newline = lex_newline_and_comments();
    let opcode = lex_opcode_or_jump_label();
    let hex_literal = lex_hex_number();
    let macro_invocation = lex_macro_invocation();

    macro_invocation
        .or(opcode)
        .or(hex_literal)
        .or(newline.clone())
        .repeated()
}

fn lex_macro_invocation() -> impl Parser<char, Token, Error = Simple<char>> {
    let ident = text::ident();

    ident
        .then_ignore(just('('))
        // TODO: args delimited by comma
        .then_ignore(just(')'))
        .map(|name| Token::MacroInvocation { name, args: vec![] })
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
