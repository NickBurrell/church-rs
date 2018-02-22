#[macro_use]
extern crate nom;

use nom::{digit, IResult};
use std::str::FromStr;
use std::fmt::{Display, Formatter};
use std::error::{Error};

#[derive(Debug)]
enum ChurchValue {
    Number(i16),
    Bool(bool),
}

#[derive(Debug)]
enum ChurchParseError {
    ChurchIntParseError,
    ChurchBoolParseError,
}

impl Error for ChurchParseError {
    fn description(&self) -> &str {
        match self {
            ChurchIntParseError => "Failed to parse value into integer",
            ChurchBoolParseError => "Failed to parse value into boolean",
        }
    }
}

impl Display for ChurchParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse_char_to_bool(input: &str) -> Result<bool, ChurchParseError> {
    match input {
        "#t" => Ok(true),
        "#f" => Ok(false),
        _   => Err(ChurchParseError::ChurchBoolParseError)
    }
}

impl FromStr for ChurchValue {
    type Err = ChurchParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bool_res = parse_char_to_bool(s);
        match bool_res {
            Ok(ret) => Ok(ChurchValue::Bool(ret)),
            Err(_) => {
                let int_res = i16::from_str(s);
                match int_res {
                    Ok(ret) => Ok(ChurchValue::Number(ret)),
                    Err(ret) => Err(ChurchParseError::ChurchIntParseError),
                }
            }
        }
    }
}

impl std::string::ToString for ChurchValue {
    fn to_string(&self) -> std::string::String {
        match self {
            &ChurchValue::Number(out) => out.to_string(),
            &ChurchValue::Bool(out) => out.to_string(),
        }
    }
}

named!(parse_number<&str, Result<ChurchValue, ChurchParseError>>,
       map!(digit, FromStr::from_str)
);

named!(parse_bool<&str, Result<ChurchValue, ChurchParseError>>,
       map!(alt!(tag!("#t") | tag!("#f")), FromStr::from_str)
);

named!(church_char<&str, char>,
       one_of!("!#$%&|*=-/:<=>?@^_~")
);

named!(church_parse<&str, Result<ChurchValue, ChurchParseError>>,
       alt!(parse_number | parse_bool)
);

fn read_expr(input: &str) -> String {
    match church_parse(input) {
        IResult::Done(_, out) => out.unwrap().to_string(),
        IResult::Error(_) => String::from("ERROR"),
        _ => String::from("ERROR"),
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok().expect("FAILED");
    let res = read_expr(&input);
    println!("{}", res);
}
