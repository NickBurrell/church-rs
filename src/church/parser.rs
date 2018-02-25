use nom::{digit, IResult, alphanumeric};

use std::fmt::{Display, Formatter};
use std::error::{Error};
use std::str::FromStr;

use self::error;

//mod error;
use super::error::*;

trait ChurchErrorKind {}

pub enum ChurchValue {
    Number(i16),
    Bool(bool),
    List(Box<Vec<ChurchValue>>),
}

pub fn vec_ref_to_string<T: ToString>(v: &Vec<T>) -> String {
    let mut out_str = String::from_str("").unwrap();
    for i in 0..v.len() {
        out_str.push_str(&v[i].to_string());
        if i < v.len() - 1 {
            out_str.push_str(", ");
        }
    }
    out_str

}


impl ChurchValue {
    fn parse_string_to_bool(input: &str) -> Result<Self, ChurchParseError> {
        match input {
            "#t" => Ok(ChurchValue::Bool(true)),
            "#f" => Ok(ChurchValue::Bool(false)),
            _   => Err(ChurchParseError::ChurchBoolParseError)
        }
    }
    fn parse_string_to_i16(input: &str) -> Result<Self, ChurchParseError> {
        let parsed_int = i16::from_str(input);
        match parsed_int {
            Ok(value) => Ok(ChurchValue::Number(value)),
            Err(_) => Err(ChurchParseError::ChurchIntParseError),
        }
    }
    fn parse_vec_to_church_value<T: ToString>(v: Vec<T>) -> Result<ChurchValue, ChurchParseError> {
        let mut out_vec: Vec<ChurchValue> = Vec::new();
        let mut output_value: Result<ChurchValue, ChurchParseError> = Ok(ChurchValue::Number(0));
        for i in v.into_iter() {
            let str_ref = i.to_string();
            let elem = church_parse(&str_ref);
            let index = out_vec.len();
            match elem {
                IResult::Done(_, val) => out_vec.insert(index, val.unwrap()),
                _ => output_value = Err(ChurchParseError::ChurchListParseError),

            };
        };
        match output_value {
            Err(_) => {},
            _ => output_value = Ok(ChurchValue::List(Box::new(out_vec))),
        }
        output_value
    }
}
impl ToString for ChurchValue {
    fn to_string(&self) -> String {
        match self {
            &ChurchValue::Number(out) => out.to_string(),
            &ChurchValue::Bool(out) => out.to_string(),
            &ChurchValue::List(ref out) => vec_ref_to_string(&*out),
        }
    }
}

named!(parse_number<&str, Result<ChurchValue, ChurchParseError>>,
       map!(digit, ChurchValue::parse_string_to_i16)
);

named!(parse_bool<&str, Result<ChurchValue, ChurchParseError>>,
       map!(alt!(tag!("#t") | tag!("#f")), ChurchValue::parse_string_to_bool)
);

named!(parse_list<&str, Result<ChurchValue, ChurchParseError>>,
       do_parse!(
           begin: tag!("(") >>
           output: separated_list!(tag!(","), alphanumeric) >>
           end: tag!(")") >>
           (ChurchValue::parse_vec_to_church_value(output))
       )
);

named!(church_symbol<&str, char>,
       one_of!("!#$%&|*=-/:<=>?@^_~")
);

named!(church_parse<&str, Result<ChurchValue, ChurchParseError>>,
       alt!(alt!(complete!(parse_bool) | complete!(parse_number)) | complete!(parse_list))
);

pub fn read_expr(input: &str) -> String {
    match church_parse(input) {
        IResult::Done(_, out) => out.unwrap().to_string(),
        IResult::Error(_) => String::from("ERROR"),
        _ => String::from("ERROR"),
    }
}

pub fn eval(input: ChurchValue) -> ChurchValue {
    match input {
        ChurchValue::List(data) => ChurchValue::List(data),
        ChurchValue::Bool(data) => ChurchValue::Bool(data),
        ChurchValue::Number(data) => ChurchValue::Number(data),
    }
}

