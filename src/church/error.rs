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
pub enum ChurchEvalError<'a> {
    FunctionNotFound(&'a str, Box<Vec<String>>),
    ArgumentError(&'a str, &'a str, Box<Vec<String>>),
    TypeError(&'a str, &'a str, Box<Vec<String>>),
}

#[derive(Debug)]
pub enum ChurchError<'a> {
    ParseError(ChurchParseError),
    EvalError(ChurchEvalError<'a>),

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

impl<'a> Error for ChurchEvalError<'a> {
    fn description(&self) -> &str {
        match self {
            &ChurchEvalError::FunctionNotFound(_, _) => "Function Not Found",
            &ChurchEvalError::ArgumentError(_, _, _) => "NYI",
            &ChurchEvalError::TypeError(_, _, _) => "NYI",
        }
    }
}

impl<'a> Display for ChurchEvalError<'a> {
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

unsafe impl<'a> ::std::marker::Sync for ChurchEvalError<'a> {}
