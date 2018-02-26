use std::fmt::{Display, Formatter};
use std::error::{Error};

use super::utils::vec_to_string;
use super::parser::ChurchValue;

trait ChurchErrorTrait : Error{}

#[derive(Debug)]
pub enum ChurchParseError {
    IntParseError,
    BoolParseError,
    ListParseError,

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

quick_error! {
    #[derive(Debug)]
    pub enum ChurchEvalError {
        FunctionNotFound(fn_name: String) {
            description("Function Not Found")
            display("[!] Error: Function \"{}\" was not found.\n", &fn_name)
        }
        ArgumentError(fn_name: String, args: Box<Vec<ChurchValue>>) {
            description("Argument Error")
            display("[!] Error: Wrong arguments for function {}.\n\tRecieved: {}\n", &fn_name, vec_to_string(*args.clone()))
        }
        TypeError(fn_name: String, expected: String, actual: String) {
            description("Wrong type")
            display("[!] Error: Wrong type of argument for function {}.\n\tExpected: {}\n\tRecieved: {}", &fn_name, &expected, &actual)
        }
    }
}

unsafe impl ::std::marker::Sync for ChurchEvalError {}
