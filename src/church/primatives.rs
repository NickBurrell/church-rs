use nom::{digit, IResult, alphanumeric, is_alphanumeric};

use std::str::FromStr;
use std::clone::Clone;

use super::error::*;
use super::utils::*;
use super::eval::*;
use super::parser::read_expr;


#[derive(Debug, Eq, PartialEq)]
pub enum ChurchValue {
    Number(i16),
    Bool(bool),
    List(Box<Vec<ChurchValue>>),
    Func(String, Box<Vec<ChurchValue>>),
}

impl ChurchValue {
    pub fn parse_string_to_bool(input: &str) -> Result<Self, ChurchParseError> {
        match input {
            "#t" => Ok(ChurchValue::Bool(true)),
            "#f" => Ok(ChurchValue::Bool(false)),
            _   => Err(ChurchParseError::BoolParseError)
        }
    }
    pub fn parse_string_to_i16(input: &str) -> Result<Self, ChurchParseError> {
        let parsed_int = i16::from_str(input);
        match parsed_int {
            Ok(value) => Ok(ChurchValue::Number(value)),
            Err(_) => Err(ChurchParseError::IntParseError),
        }
    }
    pub fn parse_vec_to_value<T: ToString>(v: Vec<T>) -> Result<ChurchValue, ChurchParseError> {
        let mut out_vec: Vec<ChurchValue> = Vec::new();
        let mut output_value: Result<ChurchValue, ChurchParseError> = Ok(ChurchValue::Number(0));
        for i in v.into_iter() {
            let str_ref = i.to_string();
            let elem = read_expr(&str_ref);
            let index = out_vec.len();
            match elem {
                Ok(val) => out_vec.insert(index, val),
                _ => output_value = Err(ChurchParseError::ListParseError),

            };
        };
        match output_value {
            Err(_) => {},
            _ => output_value = Ok(ChurchValue::List(Box::new(out_vec))),
        }
        output_value
    }
    pub fn parse_vec_to_func(_args: Vec<&str>) -> Result<ChurchValue, ChurchParseError> {
        let mut arg_list: Vec<ChurchValue> = Vec::new();

        let fn_name = _args[0].clone();
        let mut args = _args.clone();

        args.remove(0);

        for arg in args {

            let temp_val = match read_expr(arg) {
                Ok(value) => Ok(eval_statement(value)),
                _ => Err(ChurchParseError::ParseError),
            };

            match temp_val {
                Ok(_val) => {
                    match _val {
                        Ok(val) => arg_list.push(val),
                        Err(_) => return Err(ChurchParseError::ParseError),
                    }
}
                Err(err) => {
                        return Err(err);
                },
            }
        }
        Ok(ChurchValue::Func(fn_name.to_string(), Box::new(arg_list)))
    }
}


unsafe impl ::std::marker::Sync for ChurchValue {}

impl ToString for ChurchValue {
    fn to_string(&self) -> String {
        match self {
            &ChurchValue::Number(out) => out.to_string(),
            &ChurchValue::Bool(out) => out.to_string(),
            &ChurchValue::List(ref out) => vec_ref_to_string(&*out, ", "),
            &ChurchValue::Func(ref fn_name, ref args) => {
                let mut out_str = fn_name.clone();
                out_str.push_str(vec_ref_to_string(args, " ").as_str());
                out_str
            }
        }
    }
}

impl Clone for ChurchValue {
    fn clone(&self) -> ChurchValue {
        match self {
            &ChurchValue::Number(val) => ChurchValue::Number(val),
            &ChurchValue::Bool(val) => ChurchValue::Bool(val),
            &ChurchValue::List(ref val) => ChurchValue::List(val.clone()),
            &ChurchValue::Func(ref fn_name, ref args) => ChurchValue::Func(fn_name.to_owned(), args.clone()),
        }
    }
}
