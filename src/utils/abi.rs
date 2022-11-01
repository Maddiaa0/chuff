use serde::{Deserialize, Serialize};
use std::fmt;

use crate::span::Spanned;

/// Ripped from huff-rss
/// Module that contains helper functions to parse ABI types

/// #### Function
///
/// A function definition.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Function {
    /// The function name
    pub name: String,
    /// The function inputs
    pub inputs: Vec<Spanned<FunctionParam>>,
    /// The function outputs
    pub outputs: Vec<Spanned<FunctionParam>>,
    /// Constant
    pub constant: bool,
    /// The state mutability
    pub state_mutability: Spanned<FunctionType>,
}

/// Function Types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// #### Event
///
/// An Event definition.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Event {
    /// The event name
    pub name: String,
    /// The event inputs
    pub inputs: Vec<Spanned<EventParam>>,
    /// Anonymity
    pub anonymous: bool,
}

/// #### EventParam
///
/// Event parameters.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct EventParam {
    /// The parameter name
    pub name: String,
    /// The parameter type
    pub kind: FunctionParamType,
    /// If the parameter is indexed
    pub indexed: bool,
}

/// #### Error
///
/// An Error definition.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Error {
    /// The error name
    pub name: String,
    /// The error inputs
    pub inputs: Vec<Spanned<FunctionParam>>,
}

/// #### Constructor
///
/// The contract constructor
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Constructor {
    /// Contstructor inputs
    pub inputs: Vec<FunctionParam>,
}

/// #### FunctionParam
///
/// A generic function parameter
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct FunctionParam {
    /// The function parameter name
    pub name: String,
    /// The function parameter type
    pub kind: FunctionParamType,
    /// The internal type of the parameter
    pub internal_type: Option<String>,
}

/// #### FunctionParamType
///
/// The type of a function parameter
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum FunctionParamType {
    /// An address
    Address,
    /// Bytes
    Bytes,
    /// A signed integer
    Int(usize),
    /// An unsigned integer
    Uint(usize),
    /// A boolean
    Bool,
    /// A String
    String,
    /// Array ; uint256[2][] => Array(Uint(256), [2, 0])
    Array(Box<FunctionParamType>, Vec<usize>),
    /// Fixed number of bytes
    FixedBytes(usize),
    /// A tuple of parameters
    Tuple(Vec<FunctionParamType>),
}

impl FunctionParamType {
    /// Print a function parameter type to a formatter
    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FunctionParamType::Address => write!(f, "address"),
            FunctionParamType::Bytes => write!(f, "bytes"),
            FunctionParamType::Int(size) => write!(f, "int{size}"),
            FunctionParamType::Uint(size) => write!(f, "uint{size}"),
            FunctionParamType::Bool => write!(f, "bool"),
            FunctionParamType::String => write!(f, "string"),
            FunctionParamType::Array(fpt, sizes) => write!(
                f,
                "{}{}",
                fpt,
                sizes
                    .iter()
                    .map(|s| (!s.eq(&0))
                        .then(|| format!("[{s}]"))
                        .unwrap_or_else(|| "[]".to_string()))
                    .collect::<Vec<_>>()
                    .join("")
            ),
            FunctionParamType::FixedBytes(size) => write!(f, "bytes{size}"),
            FunctionParamType::Tuple(inner) => write!(
                f,
                "({})",
                inner
                    .iter()
                    .map(|fpt| fpt.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
    /// Checks if the param type should be designated as "memory" for solidity interface
    /// generation.
    pub fn is_memory_type(&self) -> bool {
        matches!(
            self,
            FunctionParamType::Bytes
                | FunctionParamType::String
                | FunctionParamType::Tuple(_)
                | FunctionParamType::Array(_, _)
        )
    }
}

impl fmt::Debug for FunctionParamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}

impl fmt::Display for FunctionParamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}

impl FunctionParamType {
    /// Convert string to type
    pub fn convert_string_to_type(string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input = string.to_string().to_lowercase();
        let split_input: Vec<&str> = input.split('[').collect();
        if split_input.len() > 1 {
            let mut cleaned: Vec<String> = split_input
                .iter()
                .map(|x| x.replace(']', ""))
                .map(|x| if x.is_empty() { "0".to_owned() } else { x })
                .collect();
            let func_type = FunctionParamType::convert_string_to_type(&cleaned.remove(0))?;
            let sizes: Vec<usize> = cleaned
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            return Ok(Self::Array(Box::new(func_type), sizes));
        }
        if input.starts_with("uint") {
            // Default to 256 if no size
            let size = match input.get(4..input.len()) {
                Some(s) => match s.is_empty() {
                    false => s.parse::<usize>().unwrap(),
                    true => 256,
                },
                None => 256,
            };
            return Ok(Self::Uint(size));
        }
        if input.starts_with("int") {
            // Default to 256 if no size
            let size = match input.get(3..input.len()) {
                Some(s) => match s.is_empty() {
                    false => s.parse::<usize>().unwrap(),
                    true => 256,
                },
                None => 256,
            };
            return Ok(Self::Int(size));
        }
        if input.starts_with("bytes") && input.len() != 5 {
            let size = input.get(5..input.len()).unwrap().parse::<usize>().unwrap();
            return Ok(Self::FixedBytes(size));
        }
        if input.starts_with("bool") {
            return Ok(Self::Bool);
        }
        if input.starts_with("address") {
            return Ok(Self::Address);
        }
        if input.starts_with("string") {
            return Ok(Self::String);
        }
        if input == "bytes" {
            Ok(Self::Bytes)
        } else {
            Err(format!(
                "Failed to create FunctionParamType from string: {}",
                string
            ))?
        }
    }
}

impl From<&str> for FunctionParamType {
    fn from(string: &str) -> Self {
        FunctionParamType::convert_string_to_type(string).unwrap()
    }
}

impl From<String> for FunctionParamType {
    fn from(string: String) -> Self {
        FunctionParamType::convert_string_to_type(&string).unwrap()
    }
}
