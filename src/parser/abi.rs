// TODO:

// /// Lex ABI Function
// ///
// /// An abi function is a solidity function selector that is typically located at the top of a
// /// Huff file.
// /// They exist in the form: `function <name>(<args>) <pure|view| > <public|external| > returns(<type>)
// fn lex_abi_function() -> impl Parser<char, Token, Error = Simple<char>> + Clone {
//     // Bases
//     let key = |w| text::keyword(w).padded();

//     // Sub parsers
//     let function_type = lex_function_type();
//     let args_parser = lex_abi_args();

//     key("function")
//         .ignore_then(text::ident())
//         .then_ignore(just('('))
//         // TODO: parse args
//         .ignore_then(just(')'))
//         .ignore_then(function_type)
//         .map(|name| {
//             // TODO: parse all of the values of the function

//             // TODO: fill in
//             Token::AbiFunction()
//         })
// }

// fn lex_function_type() -> impl Parser<char, FunctionType, Error = Simple<char>> {
//     let key = |w| text::keyword(w).padded();

//     key("view")
//         .to(FunctionType::View)
//         .or(key("payable").to(FunctionType::Payable))
//         .or(key("nonpayable").to(FunctionType::NonPayable))
//         .or(key("pure").to(FunctionType::Pure))
// }

// /// Lex ABI Event
// ///
// /// An abi event is a solidity event selector that is typically located at the top of a
// /// Huff file.
// /// They exist in the form: `event <name>(<args>)`
// fn lex_abi_event() -> impl Parser<char, Token, Error = Simple<char>> + Clone {}

// fn lex_abi_error() -> impl Parser<char, Token, Error = Simple<char>> + Clone {}

// /// Lex ABI args
// ///
// /// Lex abi args that match that of a solidity function signature
// /// Uses lex solidity type to determine the validity of the type
// fn lex_abi_args() -> impl Parser<char, FunctionParamType, Error = Simple<char>> + Clone {}

// /// Lex int type
// fn lex_int_type() -> impl Parser<char, FunctionParamType, Error = Simple<char>> + Clone {
//     let key = |w| text::keyword(w);

//     key("int")
//         .ignore_then(text::digits(10))
//         // TODO: change to map and disallow non power of 2 items - check unwrap here
//         .map(|size: String| FunctionParamType::Int(size.parse().unwrap()))
// }

// /// Lex uint type
// fn lex_uint_type() -> impl Parser<char, FunctionParamType, Error = Simple<char>> + Clone {
//     let key = |w| text::keyword(w);

//     key("uint")
//         .ignore_then(text::digits(10))
//         // TODO: change to map and disallow non power of 2 items - check unwrap here
//         .map(|size: String| FunctionParamType::Int(size.parse().unwrap()))
// }

// /// Lex ABI Type
// ///
// /// Lex a solidity type
// fn lex_solidity_type() -> impl Parser<char, Token, Error = Simple<char>> + Clone {}
