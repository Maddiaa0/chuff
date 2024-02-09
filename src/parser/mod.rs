use chumsky::prelude::*;
use std::hash::Hash;

// TODO: parse constructor

use crate::{
    lexer::token::{Literal, Token},
    span::{Span, Spanned},
    utils::{
        abi::{
            Constructor, Error, Event, EventParam, Function, FunctionParam, FunctionParamType,
            FunctionType,
        },
        ast::TableKind,
        bytes_util::bytes32_to_string,
        opcodes::Opcode,
        types::PrimitiveEVMType,
    },
};

/// Public entry point to the ast parser
pub fn parser() -> impl Parser<Token, Vec<Spanned<Ast>>, Error = Simple<Token>> {
    Ast::parser().repeated().at_least(1).then_ignore(end())
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Ast {
    ParsingError {
        token: Token,
        message: String,
    },
    FileInclude {
        path: String,
    },
    ConstantDefinition {
        name: String,
        value: ConstantValue,
    },
    MacroDefinition {
        name: String,
        macro_type: Spanned<MacroType>,
        takes: Spanned<usize>,
        returns: Spanned<usize>,
        statements: Vec<Spanned<MacroBody>>,
        args: Args,
    },
    TableDefinition {
        name: String,
        kind: TableKind,
        statements: Vec<Spanned<TableStatements>>,
    },

    // Abi types
    AbiFunction(Function),
    AbiEvent(Event),
    AbiError(Error),
    AbiConstructor(Constructor),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TableStatements {
    JumpLabel(String),
    Code(String),
    // Include so that the compiler does not fail on an unexpected input
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MacroType {
    Macro,
    Fn,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ConstantValue {
    Literal(Literal),
    FreeStoragePointer,
}

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

    UnexpectedToken(String),
}

pub type Args = Vec<Spanned<Arg>>;
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Arg {
    Valid(String),
    Invalid,
}

impl Ast {
    /// Top level Parser
    ///
    /// Attempts to parse, file inclusions, macros, fns, constants, abi events, errors, tables
    /// etc.
    ///
    /// Each top level item has its OWN parser that will attempt to match the current token
    /// in the stream.
    ///
    /// If any error occurs at this level, the parser will attempt to recover to the next DEFINE or INCLUDES token.
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let define_parser = Self::parse_define();
        let include_parser = Self::parse_include();

        define_parser.or(include_parser)
        // .or(any().map_with_span(|token, span| {
        //     (
        //         Self::ParsingError {
        //             token,
        //             message: "Expected #define or #include".to_string(),
        //         },
        //         span,
        //     )
        // }))
    }

    /// Nested Parser
    ///
    /// Utility function for when a smaller parser is enclosed by delimiters, a recovery strategy is automatically implemented based on
    /// the provided delimiters
    fn nested_parser<'a, T: 'a>(
        parser: impl Parser<Token, T, Error = Simple<Token>> + Clone + 'a,
        open_delimiter: Token,
        close_delimiter: Token,
        f: impl Fn(Span) -> T + Clone + 'a,
    ) -> impl Parser<Token, T, Error = Simple<Token>> + Clone + 'a {
        parser
            .delimited_by(just(open_delimiter.clone()), just(close_delimiter.clone()))
            .recover_with(nested_delimiters(
                open_delimiter,
                close_delimiter,
                [
                    (Token::OpenParen, Token::CloseParen),
                    (Token::OpenBrace, Token::CloseBrace),
                    (Token::OpenBracket, Token::CloseBracket),
                ],
                f,
            ))
            .boxed()
    }

    // Parse high level functions

    fn parse_include() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let extract_string = Self::extract_string();

        just(Token::Include)
            .ignore_then(extract_string.or_else(|_| Ok("____PARSING_ERROR".to_string())))
            .validate(|string, span, emit| {
                if string == *"____PARSING_ERROR" {
                    emit(Simple::custom(span, "Expected string".to_string()))
                }
                string
            })
            .map_with_span(|str: String, span| (Self::FileInclude { path: str }, span))
        // Try and find other define / include tokens in case of failure
    }

    fn parse_define() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let table_parser = Self::table_parser();
        let macro_parser = Self::parse_macro();
        let constant_parser = Self::parse_constants();
        let abi_parser = Self::parse_abi_definition();
        let event_parser = Self::parse_abi_event_definition();
        let error_parser = Self::parse_errors();

        just(Token::Define).ignore_then(
            macro_parser
                .or(error_parser)
                .or(abi_parser)
                .or(event_parser)
                .or(table_parser)
                .or(constant_parser)
                .or(any().map_with_span(|token, span| {
                    (
                        Self::ParsingError {
                            token,
                            message: "Expected keyword".to_string(),
                        },
                        span,
                    )
                })),
        )

        // .or_else(|tok| Ok((Self::ParsingError("Unexpected".to_string()), tok.span())))
    }

    fn parse_errors() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let parse_identifier = Self::extract_ident();
        let func_params = Self::parse_abi_inputs();

        just(Token::Error)
            .ignore_then(parse_identifier)
            .then_ignore(just(Token::OpenParen))
            .then(func_params)
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|(name, inputs), span| (Self::AbiError(Error { name, inputs }), span))
    }

    /// Table Parser
    ///
    /// Parses either a jump table or a code table, both are stored as the same root type, TableDefinition
    ///
    fn table_parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let jump_table = Self::parse_jump_table();
        let code_table = Self::parse_code_table();

        jump_table.or(code_table)
    }

    fn parse_jump_table() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let table_kind = Self::parse_jump_table_kind();
        let ident = Self::extract_ident();
        let optional_parens = Self::parse_optional_paren();

        let table_contents = Self::nested_parser(
            Self::parse_jump_table_contents(),
            Token::OpenBrace,
            Token::CloseBrace,
            |span| vec![(TableStatements::Error("Unexpected token".to_string()), span)],
        );

        table_kind
            .then(ident)
            .then_ignore(optional_parens.or_not())
            .then(table_contents)
            .map_with_span(|((table_kind, name), contents), span| {
                (
                    Self::TableDefinition {
                        name,
                        kind: table_kind,
                        // TODO: parse these
                        statements: contents,
                    },
                    span,
                )
            })
    }

    fn parse_jump_table_kind() -> impl Parser<Token, TableKind, Error = Simple<Token>> + Clone {
        just(Token::JumpTable)
            .to(TableKind::JumpTable)
            .or(just(Token::JumpTablePacked).to(TableKind::JumpTablePacked))
    }

    fn parse_optional_paren() -> impl Parser<Token, (), Error = Simple<Token>> + Clone {
        just(Token::OpenParen)
            .ignore_then(just(Token::CloseParen))
            .ignore_then(just(Token::Assign))
            .ignored()
    }

    fn parse_jump_table_contents(
    ) -> impl Parser<Token, Vec<Spanned<TableStatements>>, Error = Simple<Token>> + Clone {
        let ident = Self::extract_ident();
        let parse_num = Self::extract_number();
        let parse_lit = Self::extract_literal();

        ident
            .map_with_span(|ident, span| (TableStatements::JumpLabel(ident), span))
            .or(parse_num.map_with_span(|num, span| {
                (
                    TableStatements::Error(format!("Number not expected: {num}")),
                    span,
                )
            }))
            .or(parse_lit.map_with_span(|_literal, span| {
                (
                    TableStatements::Error("Literal not expected".to_string()),
                    span,
                )
            }))
            .repeated()
    }

    fn parse_code_table() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let extract_code_table_code = Self::extract_code_table();
        let ident = Self::extract_ident();
        let optional_parens = Self::parse_optional_paren();

        let code_table =
            extract_code_table_code.map_with_span(|code, span| (TableStatements::Code(code), span));

        let code_table_parser =
            Self::nested_parser(code_table, Token::OpenBrace, Token::CloseBrace, |span| {
                (TableStatements::Error("Unexpected".to_string()), span)
            });

        just(Token::CodeTable)
            .to(TableKind::CodeTable)
            .ignore_then(ident)
            .then_ignore(optional_parens.or_not())
            .then(code_table_parser)
            .map_with_span(|(name, table_content), span| {
                (
                    Self::TableDefinition {
                        name,
                        kind: TableKind::CodeTable,
                        statements: vec![table_content],
                    },
                    span,
                )
            })
    }

    fn parse_constants() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let parse_identifier = Self::extract_ident();
        let constant_value = Self::parse_constant_value();

        just(Token::Constant)
            .ignore_then(parse_identifier)
            .then_ignore(just(Token::Assign))
            .then(constant_value)
            .map_with_span(|(name, value), span| (Self::ConstantDefinition { name, value }, span))
    }

    fn parse_abi_event_definition(
    ) -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let event_args = Self::parse_event_inputs();
        let ident = Self::extract_ident();

        let event_body =
            Self::nested_parser(event_args, Token::OpenParen, Token::CloseParen, |span| {
                vec![(
                    EventParam {
                        name: "error".to_string(),
                        indexed: false,
                        // TODO: include erroneous
                        kind: FunctionParamType::Address,
                    },
                    span,
                )]
            });

        just(Token::Event)
            .ignore_then(ident)
            .then(event_body)
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
        let parse_identifier = Self::extract_ident();
        let parse_return_types = Self::parse_return_type();
        let parse_visibility = Self::parse_abi_visibility();

        let parse_abi_args = Self::parse_abi_inputs().or_not();
        let nested_parse_abi_args = Self::nested_parser(
            parse_abi_args,
            Token::OpenParen,
            Token::CloseParen,
            |_span| None,
        );

        just(Token::Function)
            .ignore_then(parse_identifier)
            .then(nested_parse_abi_args)
            .then(parse_visibility)
            .then(parse_return_types.or_not())
            // TODO: parse possible return types
            .map_with_span(|(((name, inputs), state_mutability), return_types), span| {
                (
                    Self::AbiFunction(Function {
                        name,
                        inputs: inputs.unwrap_or_default(),
                        outputs: return_types.unwrap_or_default(),
                        constant: false,
                        // TODO SET
                        state_mutability,
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
        let abi_outputs = Self::nested_parser(
            Self::parse_abi_inputs(),
            Token::OpenParen,
            Token::CloseParen,
            |_span| Vec::new(),
        );

        just(Token::Returns).ignore_then(abi_outputs)
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
        let ident = Self::extract_ident();

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
                            internal_type,
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
        let ident = Self::extract_ident();

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
            .map(ConstantValue::Literal)
            .or(parse_fsp.to(ConstantValue::FreeStoragePointer))
    }

    fn parse_fsp() -> impl Parser<Token, (), Error = Simple<Token>> + Clone {
        just(Token::FreeStoragePointer)
            .ignore_then(just(Token::OpenParen).ignore_then(just(Token::CloseParen)))
            .ignored()
    }

    fn parse_macro() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token>> + Clone {
        let parse_macro_type = Self::parse_macro_type();
        let parse_identifier = Self::extract_ident();

        let parse_args = Self::nested_parser(
            Self::parse_args(),
            Token::OpenParen,
            Token::CloseParen,
            |span| vec![(Arg::Invalid, span)],
        );

        let parse_takes = Self::parse_takes();
        let parse_returns = Self::parse_returns();

        let macro_body = Self::nested_parser(
            Self::parse_macro_body(),
            Token::OpenBrace,
            Token::CloseBrace,
            |_span| Vec::new(),
        );

        parse_macro_type
            .then(parse_identifier)
            .then(parse_args)
            .then_ignore(just(Token::Assign))
            .then(parse_takes.or_not())
            .then(parse_returns.or_not())
            .then(macro_body)
            // TODO: recover with open and close delimiters
            .map_with_span(
                |(((((macro_type, name), args), takes), returns), body), span| {
                    (
                        Self::MacroDefinition {
                            name,
                            macro_type,
                            // Todo: more suitable default for takes and returns
                            takes: takes.unwrap_or((0, 0..0)),
                            returns: returns.unwrap_or((0, 0..0)),
                            statements: body,
                            args,
                        },
                        span,
                    )
                },
            )
    }

    fn parse_macro_type() -> impl Parser<Token, Spanned<MacroType>, Error = Simple<Token>> + Clone {
        just(Token::Macro)
            .to(MacroType::Macro)
            .or(just(Token::Fn).to(MacroType::Fn))
            .map_with_span(|tok, span| (tok, span))
    }

    // TODO: Morph parse takes and parse returns into one
    fn parse_takes() -> impl Parser<Token, Spanned<usize>, Error = Simple<Token>> + Clone {
        let number = Self::extract_number();

        just(Token::Takes)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(number.or_not())
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|num_takes: Option<usize>, span| {
                let takes = num_takes.unwrap_or(0);
                (takes, span)
            })
    }
    fn parse_returns() -> impl Parser<Token, Spanned<usize>, Error = Simple<Token>> + Clone {
        let number = Self::extract_number();

        just(Token::Returns)
            .ignore_then(just(Token::OpenParen))
            .ignore_then(number.or_not())
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|num_returns: Option<usize>, span| {
                let takes = num_returns.unwrap_or(0);
                (takes, span)
            })
    }

    /// Parse arguments to macro calls
    ///
    /// Note: This cannot be used in abi function calls
    fn parse_args() -> impl Parser<Token, Args, Error = Simple<Token>> + Clone {
        let ident = Self::extract_ident();
        let num = Self::extract_number();
        let literal = Self::extract_literal();

        ident
            .map_with_span(|arg, span| (Arg::Valid(arg), span))
            // Handle invalid cases
            .or(num.map_with_span(|_, span| (Arg::Invalid, span)))
            .or(literal.map_with_span(|_, span| (Arg::Invalid, span)))
            .then_ignore(just(Token::Comma).or_not())
            .repeated()
    }

    fn parse_macro_body(
    ) -> impl Parser<Token, Vec<Spanned<MacroBody>>, Error = Simple<Token>> + Clone {
        let opcode = Self::extract_opcode();
        let macro_invocation = Self::parse_macro_invocation();
        let builtin_invocation = Self::parse_builtin_invocation();
        let jump_label = Self::parse_jump_label();
        let arg_invocation = Self::parse_arg_invocation();
        let hex_literal = Self::parse_hex_literal();

        let extract_num = Self::extract_number();

        let unexpected_macro_chars = extract_num
            .map_with_span(|num: usize, span| (MacroBody::UnexpectedToken(num.to_string()), span));
        let unexpected_keyword = filter::<Token, _, Simple<Token>>(|token| {
            // TODO: create a vector of keywords invalid inside a macro
            let invalid_keywords = [Token::Macro, Token::Function, Token::Storage];
            invalid_keywords.contains(token)
        })
        .map_with_span(|token: Token, span| (MacroBody::UnexpectedToken(token.to_string()), span));

        opcode
            .map_with_span(|tok, span| (MacroBody::Opcode(tok), span))
            .or(macro_invocation)
            .or(hex_literal)
            .or(arg_invocation)
            .or(builtin_invocation)
            .or(jump_label)
            .or(unexpected_macro_chars)
            .or(unexpected_keyword)
            .repeated()
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
        let ident = Self::extract_ident();

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
        let ident = Self::extract_ident();

        just(Token::LeftAngle)
            .ignore_then(ident)
            .then_ignore(just(Token::RightAngle))
            .map_with_span(|arg, span| (MacroBody::ArgsInvocation(arg), span))
    }

    fn parse_builtin_invocation(
    ) -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone {
        let builtin_ident = Self::extract_builtin_ident();
        let parse_args = Self::parse_args();

        builtin_ident
            .then_ignore(just(Token::OpenParen))
            .then(parse_args)
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|(name, args), span| (MacroBody::BuiltinInvocation { name, args }, span))
    }

    fn parse_macro_invocation(
    ) -> impl Parser<Token, Spanned<MacroBody>, Error = Simple<Token>> + Clone {
        let ident = Self::extract_ident();
        let parse_args = Self::parse_args();

        ident
            .then_ignore(just(Token::OpenParen))
            .then(parse_args)
            .then_ignore(just(Token::CloseParen))
            .map_with_span(|(name, args), span| (MacroBody::MacroInvocation { name, args }, span))
    }

    // Utility functions to extract data from lexing tokens

    /// Parsers to extract nested information from the tokens
    fn extract_string() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::Str(str) => str }.labelled("string")
    }

    fn extract_ident() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::Ident(str) => str}.labelled("identifier")
    }

    fn extract_opcode() -> impl Parser<Token, Opcode, Error = Simple<Token>> + Clone {
        select! {Token::Opcode(opcode) => opcode}.labelled("opcode")
    }

    fn extract_builtin_ident() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::BuiltinFunction(name) => name}.labelled("builtin")
    }

    fn extract_literal() -> impl Parser<Token, Literal, Error = Simple<Token>> + Clone {
        select! { Token::Literal(lit) => lit }.labelled("hex_literal")
    }

    fn extract_number() -> impl Parser<Token, usize, Error = Simple<Token>> + Clone {
        select! { Token::Num(val) => val}.labelled("number")
    }

    // TODO: handle tuple definitions
    fn extract_primitive() -> impl Parser<Token, FunctionParamType, Error = Simple<Token>> + Clone {
        let fixed_primitive = Self::extract_fixed_primitive();
        let array_primitive = Self::extract_array_primitive();

        fixed_primitive.or(array_primitive)
    }

    fn extract_fixed_primitive(
    ) -> impl Parser<Token, FunctionParamType, Error = Simple<Token>> + Clone {
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

    // TODO: shorten
    fn extract_array_primitive(
    ) -> impl Parser<Token, FunctionParamType, Error = Simple<Token>> + Clone {
        select! { Token::ArrayType(primitive, array) => (primitive, array)}
            .labelled("array_primitive")
            .map(|(primitive, arr)| match primitive {
                PrimitiveEVMType::Address => {
                    FunctionParamType::Array(Box::new(FunctionParamType::Address), arr)
                }
                PrimitiveEVMType::DynBytes => {
                    FunctionParamType::Array(Box::new(FunctionParamType::Bytes), arr)
                }
                PrimitiveEVMType::String => {
                    FunctionParamType::Array(Box::new(FunctionParamType::String), arr)
                }
                PrimitiveEVMType::Bool => {
                    FunctionParamType::Array(Box::new(FunctionParamType::Bool), arr)
                }
                PrimitiveEVMType::Int(v) => {
                    FunctionParamType::Array(Box::new(FunctionParamType::Int(v)), arr)
                }
                PrimitiveEVMType::Uint(v) => {
                    FunctionParamType::Array(Box::new(FunctionParamType::Uint(v)), arr)
                }
                PrimitiveEVMType::Bytes(v) => {
                    FunctionParamType::Array(Box::new(FunctionParamType::FixedBytes(v)), arr)
                }
            })
    }

    fn extract_code_table() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        let code_from_literal = Self::extract_code_from_literal();
        let code = Self::extract_code();

        code_from_literal.or(code)
    }

    fn extract_code_from_literal() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::Literal(lit) => bytes32_to_string(&lit, false)}.labelled("codetable_code")
    }

    fn extract_code() -> impl Parser<Token, String, Error = Simple<Token>> + Clone {
        select! { Token::Code(string) => string}.labelled("codetable_code")
    }
}
