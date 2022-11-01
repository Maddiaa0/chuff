use chumsky::prelude::*;
use std::hash::Hash;

use crate::{
    lexer::token::{Literal, Token},
    span::Spanned,
    utils::{
        abi::{
            Constructor, Error, Event, EventParam, Function, FunctionParam, FunctionParamType,
            FunctionType,
        },
        opcodes::Opcode,
        types::PrimitiveEVMType,
    },
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
        value: ConstantValue,
    },

    // TODO fill this out with the required info
    MacroDefinition {
        name: String,
        takes: u32,
        returns: u32,
        body: Vec<Spanned<MacroBody>>,
        args: Args,
    },

    // Abi types
    AbiFunction(Function),
    AbiEvent(Event),
    AbiError(Error),
    AbiConstructor(Constructor),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ConstantValue {
    Literal(Literal),
    FreeStoragePointer,
}

// TOOD: include args
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MacroBody {
    Opcode(Opcode),

    // TODO give each of these names and args
    MacroInvocation { name: String, args: Args },
    ArgsInvocation(String),
    BuiltinInvocation { name: String, args: Args },
    JumpLabel(String),
    JumpLabelDest(String),
    HexLiteral(Literal),
}

pub type Args = Vec<Spanned<String>>;

impl Statement {
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let include_parser = Self::parse_include();
        let macro_parser = Self::parse_macro();
        let constant_parser = Self::parse_constants();
        let abi_parser = Self::parse_abi_definition();
        let event_parser = Self::parse_abi_event_definition();
        let error_parser = Self::parse_errors();

        include_parser
            .or(error_parser)
            .or(abi_parser)
            .or(event_parser)
            .or(macro_parser)
            .or(constant_parser)
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

    fn parse_errors() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let parse_identifier = Self::ident_parser();
        let func_params = Self::parse_abi_inputs();

        just(Token::Define)
            .ignore_then(just(Token::Error))
            .ignore_then(parse_identifier)
            .then_ignore(just(Token::OpenParen))
            .then(func_params)
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|(name, inputs), span| (Self::AbiError(Error { name, inputs }), span))
    }

    fn parse_constants() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let parse_identifier = Self::ident_parser();
        let constant_value = Self::parse_constant_value();

        just(Token::Define)
            .ignore_then(just(Token::Constant))
            .ignore_then(parse_identifier)
            .then_ignore(just(Token::Assign))
            .then(constant_value)
            .map_with_span(|(name, value), span| (Self::ConstantDefinition { name, value }, span))
    }

    // TODO: parse event args
    fn parse_abi_event_definition(
    ) -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let event_args = Self::parse_event_inputs();
        let ident = Self::ident_parser();

        just(Token::Define)
            .ignore_then(just(Token::Event))
            .ignore_then(ident)
            .then_ignore(just(Token::OpenParen))
            .then(event_args)
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|(name, inputs), span| {
                (
                    Self::AbiEvent(Event {
                        name,
                        inputs,
                        // TODO: work out what this is
                        anonymous: false,
                    }),
                    span,
                )
            })
    }

    fn parse_abi_definition() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let parse_identifier = Self::ident_parser();
        let parse_abi_args = Self::parse_abi_inputs();
        let parse_return_types = Self::parse_return_type();
        let parse_visibility = Self::parse_abi_visibility();

        just(Token::Define)
            .ignore_then(just(Token::Function))
            .ignore_then(parse_identifier)
            .then_ignore(just(Token::OpenParen))
            .then(parse_abi_args.clone().or_not())
            .then_ignore(just(Token::CloseParen))
            .then(parse_visibility)
            .then(parse_return_types.or_not())
            // TODO: parse possible return types
            .map_with_span(|(((name, inputs), state_mutability), return_types), span| {
                (
                    Self::AbiFunction(Function {
                        name,
                        inputs: inputs.unwrap_or(vec![]),
                        outputs: return_types.unwrap_or(vec![]),
                        constant: false,
                        // TODO SET
                        state_mutability: state_mutability,
                    }),
                    span,
                )
            })
    }

    fn parse_abi_visibility(
    ) -> impl Parser<Token, Spanned<FunctionType>, Error = Simple<Token>> + Clone {
        just(Token::View)
            .map_with_span(|_, span| (FunctionType::View, span))
            .or(just(Token::Payable).map_with_span(|_, span| (FunctionType::Payable, span)))
            .or(just(Token::NonPayable).map_with_span(|_, span| (FunctionType::NonPayable, span)))
            .or(just(Token::Pure).map_with_span(|_, span| (FunctionType::Pure, span)))
    }

    fn parse_return_type(
    ) -> impl Parser<Token, Vec<Spanned<FunctionParam>>, Error = Simple<Token>> + Clone {
        let abi_outputs = Self::parse_abi_inputs();

        just(Token::Returns)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(abi_outputs)
            .then_ignore(just(Token::CloseParen))
    }

    // TODO: handle arrays / tuple definitions
    fn extract_primitive() -> impl Parser<Token, FunctionParamType, Error = Simple<Token>> + Clone {
        select! {Token::PrimitiveType(prim_type) => prim_type}
            .labelled("primitive_type")
            .map(|token| match token {
                PrimitiveEVMType::Address => FunctionParamType::Address,
                PrimitiveEVMType::DynBytes => FunctionParamType::Bytes,
                PrimitiveEVMType::Bool => FunctionParamType::Bool,
                PrimitiveEVMType::String => FunctionParamType::String,
                PrimitiveEVMType::Int(v) => FunctionParamType::Int(v),
                PrimitiveEVMType::Bytes(v) => FunctionParamType::FixedBytes(v),
                PrimitiveEVMType::Uint(v) => FunctionParamType::Uint(v),
            })
    }

    fn parse_parameter_kind() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        just(Token::Memory)
            .map(|_| "memory".to_string())
            .or(just(Token::Storage).map(|_| "storage".to_string()))
            .or(just(Token::Calldata).map(|_| "calldata".to_string()))
    }

    // // TODO: change return type
    /// Parse a function abi input
    ///
    /// This parses a grammar in the following format
    /// (<type> <kind(memory|storage)>? <name>)
    fn parse_abi_inputs(
    ) -> impl Parser<Token, Vec<Spanned<FunctionParam>>, Error = Simple<Token>> + Clone {
        let primitive = Self::extract_primitive();
        let param_kind = Self::parse_parameter_kind();
        let ident = Self::ident_parser();

        primitive
            .then(param_kind.or_not())
            .then(ident.or_not())
            .map_with_span(
                // TODO: workout what the internal type field is in huff-rs
                |((param_kind, internal_type), name), span| {
                    (
                        FunctionParam {
                            name: name.unwrap_or("".to_string()),
                            // TODO: handle arrays / tuples
                            kind: param_kind,
                            // TODO: implement this properly
                            internal_type: internal_type,
                        },
                        span,
                    )
                },
            )
            .then_ignore(just(Token::Comma).or_not())
            .repeated()
    }

    fn parse_event_inputs(
    ) -> impl Parser<Token, Vec<Spanned<EventParam>>, Error = Simple<Token>> + Clone {
        let primitive = Self::extract_primitive();
        let ident = Self::ident_parser();

        primitive
            .then(just(Token::Indexed).or_not())
            .then(ident.or_not())
            .map_with_span(|((kind, indexed), name), span| {
                (
                    EventParam {
                        name: name.unwrap_or("".to_string()),
                        kind,
                        indexed: indexed.is_some(),
                    },
                    span,
                )
            })
            .then_ignore(just(Token::Comma).or_not())
            .repeated()
    }

    fn parse_constant_value() -> impl Parser<Token, ConstantValue, Error = Simple<Token>> + Clone {
        let parse_literal = Self::extract_literal();
        let parse_fsp = Self::parse_fsp();

        parse_literal
            .map(|lit| ConstantValue::Literal(lit))
            .or(parse_fsp.to(ConstantValue::FreeStoragePointer))
    }

    fn parse_fsp() -> impl Parser<Token, (), Error = Simple<Token>> + Clone {
        just(Token::FreeStoragePointer)
            .ignore_then(just(Token::OpenParen).ignore_then(just(Token::CloseParen)))
            .ignored()
    }

    fn parse_macro() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let parse_identifier = Self::ident_parser();
        let parse_args = Self::parse_args();
        let macro_body = Self::parse_macro_body();

        just(Token::Define)
            .ignore_then(just(Token::Macro))
            .ignore_then(parse_identifier)
            .then_ignore(just(Token::OpenParen))
            .then(parse_args)
            .then_ignore(just(Token::CloseParen))
            .then_ignore(just(Token::Assign))
            // TODO: handle takes / returns
            .then_ignore(just(Token::OpenBrace))
            // TODO: remove newlines altogether?
            .then_ignore(just(Token::Newline).repeated().or_not())
            .then(macro_body)
            .then_ignore(just(Token::Newline).repeated().or_not())
            .then_ignore(just(Token::CloseBrace))
            // TODO: recover with open and close delimiters
            .map_with_span(|((name, args), body), span| {
                (
                    Self::MacroDefinition {
                        name,
                        takes: 0,
                        returns: 0,
                        body,
                        args,
                    },
                    span,
                )
            })
    }

    /// Parse arguments to macro calls
    ///
    /// Note: This cannot be used in abi function calls
    fn parse_args() -> impl Parser<Token, Args, Error = Simple<Token>> + Clone {
        let ident = Self::ident_parser();

        ident
            .then_ignore(just(Token::Comma).or_not())
            .map_with_span(|arg, span| (arg, span))
            .repeated()
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
        let parse_args = Self::parse_args();

        builtin_ident
            .then_ignore(just(Token::OpenParen))
            .then(parse_args)
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|(name, args), span| (MacroBody::BuiltinInvocation { name, args }, span))
    }

    fn parse_macro_invocation(
    ) -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone {
        let ident = Self::ident_parser();
        let parse_args = Self::parse_args();

        ident
            .then_ignore(just(Token::OpenParen))
            .then(parse_args)
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|(name, args), span| (MacroBody::MacroInvocation { name, args }, span))
    }
}
