use nom::{digit, IResult, alphanumeric};

use std::fmt::{Display, Formatter};
use std::error::{Error};
use std::str::FromStr;

#[derive(Debug)]
pub enum ChurchParseError {
    ChurchIntParseError,
    ChurchBoolParseError,
    ChurchListParseError,

}
#[derive(Debug)]
pub enum ChurchEvalError {
    FunctionNotFound,
    ArgumentError,
}

impl Error for ChurchParseError {
    fn description(&self) -> &str {
        match self {
            ChurchIntParseError => "Failed to parse value into integer",
            ChurchBoolParseError => "Failed to parse value into boolean",
            ChurchListParseError => "Failed to parse values into list",
        }
    }
}

impl Display for ChurchParseError {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}
