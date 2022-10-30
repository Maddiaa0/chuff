// TODO: add builtin args etc

use phf::phf_map;

/// Built-ins in a static array
pub const BUILTINS: [&str; 8] = [
    "__tablestart",
    "__tablesize",
    "__codesize",
    "__FUNC_SIG",
    "__EVENT_HASH",
    "__ERROR",
    "__RIGHTPAD",
    "__DYN_CONSTRUCTOR_ARG",
];

pub static BUILTINS_MAP: phf::Map<&'static str, BuiltinFunctionKind> = phf_map! {
    "__tablestart"=> BuiltinFunctionKind::Tablestart,
    "__tablesize"=> BuiltinFunctionKind::Tablesize,
    "__codesize"=> BuiltinFunctionKind::Codesize,
    "__FUNC_SIG"=> BuiltinFunctionKind::FunctionSignature,
    "__EVENT_HASH"=> BuiltinFunctionKind::EventHash,
    "__ERROR"=> BuiltinFunctionKind::Error,
    "__RIGHTPAD"=> BuiltinFunctionKind::RightPad,
    "__DYN_CONSTRUCTOR_ARG"=> BuiltinFunctionKind::DynConstructorArg,
};

#[derive(Debug, Clone)]
pub enum BuiltinFunctionKind {
    /// Table size function
    Tablesize,
    /// Code size function
    Codesize,
    /// Table start function
    Tablestart,
    /// Function signature function
    FunctionSignature,
    /// Event hash function
    EventHash,
    /// Error selector function
    Error,
    /// Rightpad function
    RightPad,
    /// Dynamic constructor arg function
    DynConstructorArg,
}

impl From<String> for BuiltinFunctionKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "__tablesize" => BuiltinFunctionKind::Tablesize,
            "__codesize" => BuiltinFunctionKind::Codesize,
            "__tablestart" => BuiltinFunctionKind::Tablestart,
            "__FUNC_SIG" => BuiltinFunctionKind::FunctionSignature,
            "__EVENT_HASH" => BuiltinFunctionKind::EventHash,
            "__ERROR" => BuiltinFunctionKind::Error,
            "__RIGHTPAD" => BuiltinFunctionKind::RightPad,
            "__CODECOPY_DYN_ARG" => BuiltinFunctionKind::DynConstructorArg,
            _ => panic!("Invalid Builtin Function Kind"), /* This should never be reached,
                                                           * builtins are validated with a
                                                           * `try_from` call in the lexer. */
        }
    }
}

impl TryFrom<&String> for BuiltinFunctionKind {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, <BuiltinFunctionKind as TryFrom<&String>>::Error> {
        match value.as_str() {
            "__tablesize" => Ok(BuiltinFunctionKind::Tablesize),
            "__codesize" => Ok(BuiltinFunctionKind::Codesize),
            "__tablestart" => Ok(BuiltinFunctionKind::Tablestart),
            "__FUNC_SIG" => Ok(BuiltinFunctionKind::FunctionSignature),
            "__EVENT_HASH" => Ok(BuiltinFunctionKind::EventHash),
            "__ERROR" => Ok(BuiltinFunctionKind::Error),
            "__RIGHTPAD" => Ok(BuiltinFunctionKind::RightPad),
            "__CODECOPY_DYN_ARG" => Ok(BuiltinFunctionKind::DynConstructorArg),
            _ => Err(()),
        }
    }
}

// TODO: add spans

// A Builtin Function Call
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct BuiltinFunctionCall {
//     /// The Builtin Kind
//     pub kind: BuiltinFunctionKind,
//     /// Arguments for the builtin function call.
//     /// TODO: Maybe make a better type for this other than `Argument`? Would be nice if it pointed
//     ///       directly to the macro/table.
//     pub args: Vec<Argument>,
// }
