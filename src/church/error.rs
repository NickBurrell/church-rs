use std::fmt::{Display, Formatter};
use std::error::{Error};

use super::utils::vec_to_string;

trait ChurchErrorTrait : Error{}

#[derive(Debug)]
pub enum ChurchParseError {
    IntParseError,
    BoolParseError,
    ListParseError,

}
#[derive(Debug)]
pub enum ChurchEvalError {
    FunctionNotFound(String, Box<Vec<String>>),
    ArgumentError(String, String, Box<Vec<String>>),
    TypeError(String, String, Box<Vec<String>>),
}

#[derive(Debug)]
pub enum ChurchError {
    ParseError(ChurchParseError),
    EvalError(ChurchEvalError),

}

impl Error for ChurchParseError {
    fn description(&self) -> &str {
        match self {
            &ChurchParseError::IntParseError => "Failed to parse value into integer",
            &ChurchParseError::BoolParseError => "Failed to parse value into boolean",
            &ChurchParseError::ListParseError => "Failed to parse values into list",
        }
    }
}

impl Display for ChurchParseError {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ChurchEvalError {
    fn description(&self) -> &str {
        match self {
            &ChurchEvalError::FunctionNotFound(_, _) => "Function Not Found",
            &ChurchEvalError::ArgumentError(_, _, _) => "NYI",
            &ChurchEvalError::TypeError(_, _, _) => "NYI",
        }
    }
}

impl Display for ChurchEvalError {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match self {
            &ChurchEvalError::FunctionNotFound(ref fn_name, ref arg_list) => {
                write!(f, "[!] Error: Function {} with arguments ({}) was not found", &fn_name, vec_to_string(arg_list.to_vec()));
            },
            &ChurchEvalError::ArgumentError(_, _, _) => {
                write!(f, "{}", self);
            },
            &ChurchEvalError::TypeError(_, _, _) => {
                write!(f, "{}", self);
            }

        }
        write!(f, "{}", self)
    }
}

unsafe impl ::std::marker::Sync for ChurchEvalError {}
