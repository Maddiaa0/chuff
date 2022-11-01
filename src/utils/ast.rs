use serde::{Deserialize, Serialize};

use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use super::{bytes_util::bytes32_to_string, opcodes::Opcode};

/// A contained literal
pub type Literal = [u8; 32];

/// A File Path
///
/// Used for parsing the huff imports.
pub type FilePath = PathBuf;

/// A Huff Contract Representation
///
/// This is the representation of a contract as it is parsed from huff source code.
/// Thus, it is also the root of the AST.
///
/// For examples of Huff contracts, see the [huff-examples repository](https://github.com/huff-language/huff-examples).
#[derive(Debug, Default, Clone)]
pub struct Contract {
    /// Macro definitions
    pub macros: Vec<MacroDefinition>,
    /// Invocations of macros
    pub invocations: Vec<MacroInvocation>,
    /// File Imports
    pub imports: Vec<FilePath>,
    /// Constants
    pub constants: Arc<Mutex<Vec<ConstantDefinition>>>,
    /// Custom Errors
    pub errors: Vec<ErrorDefinition>,
    /// Functions
    pub functions: Vec<Function>,
    /// Events
    pub events: Vec<Event>,
    /// Tables
    pub tables: Vec<TableDefinition>,
}

impl Contract {
    /// Returns the first macro that matches the provided name
    pub fn find_macro_by_name(&self, name: &str) -> Option<MacroDefinition> {
        if let Some(m) = self.macros.iter().find(|m| m.name == name) {
            Some(m.clone())
        } else {
            None
        }
    }

    /// Returns the first table that matches the provided name
    pub fn find_table_by_name(&self, name: &str) -> Option<TableDefinition> {
        if let Some(t) = self.tables.iter().find(|t| t.name == name) {
            Some(t.clone())
        } else {
            None
        }
    }
}

/// An argument's location
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ArgumentLocation {
    /// Memory location
    #[default]
    Memory,
    /// Storage location
    Storage,
    /// Calldata location
    Calldata,
}

/// A function, event, or macro argument
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Argument {
    /// Type of the argument
    pub arg_type: Option<String>,
    /// Optional Argument Location
    pub arg_location: Option<ArgumentLocation>,
    /// The name of the argument
    pub name: Option<String>,
    /// Is the argument indexed? TODO: should be valid for event arguments ONLY
    pub indexed: bool,
}

/// A Function Signature
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    /// The name of the function
    pub name: String,
    /// The function signature
    pub signature: [u8; 4],
    /// The parameters of the function
    pub inputs: Vec<Argument>,
    /// The function type
    pub fn_type: FunctionType,
    /// The return values of the function
    pub outputs: Vec<Argument>,
}

/// Function Types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionType {
    /// Viewable Function
    View,
    /// Payable Function
    Payable,
    /// Non Payable Function
    NonPayable,
    /// Pure Function
    Pure,
}

impl FunctionType {
    /// Get the string representation of the function type for usage in Solidity interface
    /// generation.
    pub fn interface_mutability(&self) -> &str {
        match self {
            FunctionType::View => " view",
            FunctionType::Pure => " pure",
            _ => "", // payable / nonpayable types not valid in Solidity interfaces
        }
    }
}

/// An Event Signature
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    /// The name of the event
    pub name: String,
    /// The parameters of the event
    pub parameters: Vec<Argument>,
    /// The event hash
    pub hash: Literal,
}

/// A Table Definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableDefinition {
    /// The name of the table
    pub name: String,
    /// The table kind
    pub kind: TableKind,
    /// The table's statements
    pub statements: Vec<Statement>,
    /// Size of table
    pub size: Literal,
}

impl TableDefinition {
    /// Public associated function that instantiates a TableDefinition from a string
    pub fn new(name: String, kind: TableKind, statements: Vec<Statement>, size: Literal) -> Self {
        TableDefinition {
            name,
            kind,
            statements,
            size,
        }
    }
}

/// A Table Kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableKind {
    /// A regular jump table
    JumpTable,
    /// A packed jump table
    JumpTablePacked,
    /// A code table
    CodeTable,
}

/// A Macro Definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroDefinition {
    /// The Macro Name
    pub name: String,
    /// The macro's decorator
    pub decorator: Option<Decorator>,
    /// A list of Macro parameters
    pub parameters: Vec<Argument>,
    /// A list of Statements contained in the Macro
    pub statements: Vec<Statement>,
    /// The take size
    pub takes: usize,
    /// The return size
    pub returns: usize,
    /// Is the macro a function (outlined)?
    pub outlined: bool,
    /// Is the macro a test?
    pub test: bool,
}

impl MacroDefinition {
    /// Public associated function that instantiates a MacroDefinition.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        decorator: Option<Decorator>,
        parameters: Vec<Argument>,
        statements: Vec<Statement>,
        takes: usize,
        returns: usize,
        outlined: bool,
        test: bool,
    ) -> Self {
        MacroDefinition {
            name,
            decorator,
            parameters,
            statements,
            takes,
            returns,
            outlined,
            test,
        }
    }
}

/// A Macro Invocation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroInvocation {
    /// The Macro Name
    pub macro_name: String,
    /// A list of Macro arguments
    pub args: Vec<MacroArg>,
}

/// An argument passed when invoking a maco
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MacroArg {
    /// Macro Literal Argument
    Literal(Literal),
    /// Macro Iden String Argument
    Ident(String),
    /// An Arg Call
    ArgCall(String),
}

/// Free Storage Pointer Unit Struct
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeStoragePointer;

/// A Constant Value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstVal {
    /// A literal value for the constant
    Literal(Literal),
    /// A Free Storage Pointer
    FreeStoragePointer(FreeStoragePointer),
}

/// A Constant Definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstantDefinition {
    /// The Constant name
    pub name: String,
    /// The Constant value
    pub value: ConstVal,
}

/// An Error Definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorDefinition {
    /// The Error name
    pub name: String,
    /// The Error's selector
    pub selector: [u8; 4],
    /// The parameters of the error
    pub parameters: Vec<Argument>,
}

/// A Jump Destination
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    /// The JumpDest Name
    pub name: String,
    /// Statements Inside The JumpDest
    pub inner: Vec<Statement>,
}

/// A Builtin Function Call
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuiltinFunctionCall {
    /// The Builtin Kind
    pub kind: BuiltinFunctionKind,
    /// Arguments for the builtin function call.
    /// TODO: Maybe make a better type for this other than `Argument`? Would be nice if it pointed
    ///       directly to the macro/table.
    pub args: Vec<Argument>,
}

/// A Builtin Function Kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// A Statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statement {
    /// The type of statement
    pub ty: StatementType,
}

/// The Statement Type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatementType {
    /// A Literal Statement
    Literal(Literal),
    /// An Opcode Statement
    Opcode(Opcode),
    /// A Code Statement
    Code(String),
    /// A Macro Invocation Statement
    MacroInvocation(MacroInvocation),
    /// A Constant Push
    Constant(String),
    /// An Arg Call
    ArgCall(String),
    /// A Label
    Label(Label),
    /// A Label Reference/Call
    LabelCall(String),
    /// A built-in function call
    BuiltinFunctionCall(BuiltinFunctionCall),
}

impl Display for StatementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementType::Literal(l) => write!(f, "LITERAL: {}", bytes32_to_string(l, true)),
            StatementType::Opcode(o) => write!(f, "OPCODE: {}", o),
            StatementType::Code(s) => write!(f, "CODE: {}", s),
            StatementType::MacroInvocation(m) => {
                write!(f, "MACRO INVOCATION: {}", m.macro_name)
            }
            StatementType::Constant(c) => write!(f, "CONSTANT: {}", c),
            StatementType::ArgCall(c) => write!(f, "ARG CALL: {}", c),
            StatementType::Label(l) => write!(f, "LABEL: {}", l.name),
            StatementType::LabelCall(l) => write!(f, "LABEL CALL: {}", l),
            StatementType::BuiltinFunctionCall(b) => {
                write!(f, "BUILTIN FUNCTION CALL: {:?}", b.kind)
            }
        }
    }
}

/// A decorator tag
///
/// At the moment, the decorator tag can only be placed over test definitions. Developers
/// can use decorators to define environment variables and other metadata for their individual
/// tests.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decorator {
    /// Vector of flags passed within the decorator
    pub flags: Vec<DecoratorFlag>,
}

/// A decorator flag
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecoratorFlag {
    /// Sets the calldata of the test call transaction
    Calldata(String),
    /// Sets the value of the test call transaction
    Value(Literal),
}

impl TryFrom<&String> for DecoratorFlag {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "calldata" => Ok(DecoratorFlag::Calldata(String::default())),
            "value" => Ok(DecoratorFlag::Value(Literal::default())),
            _ => Err(()),
        }
    }
}
