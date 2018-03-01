use nom::{digit, IResult, alphanumeric, is_alphanumeric};

use std::str::FromStr;
use std::clone::Clone;

use super::error::*;
use super::utils::*;
use super::eval::*;
use super::primatives::*;
use super::tokens::*;

//type ChurchBinopFunc = Fn(ChurchValue, ChurchValue) -> Result<ChurchValue, ChurchEvalError> + 'static + Sync + Sized;

//const fn mk_church_binop<F: Sync>(f: F) -> ChurchBinopFunc
//where F: Fn(ChurchValue, ChurchValue) -> Result<ChurchValue, ChurchEvalError> + 'static {
//    Box::new(f) as ChurchBinopFunc
//}
fn is_church_char(c: char) -> bool {
    if is_alphanumeric(c as u8) {
        true
    } else {
        match church_primatives(char::to_string(&c).as_str()) {
            IResult::Done(_, _) => true,
            _ => false
        }
    }
}
named!(parse_number<&str, Result<ChurchValue, ChurchParseError>>,
       map!(digit, ChurchValue::parse_string_to_i16)
);

named!(parse_bool<&str, Result<ChurchValue, ChurchParseError>>,
       map!(alt!(tag!("#t") | tag!("#f")), ChurchValue::parse_string_to_bool)
);

named!(alphanumeric_or_bool<&str, &str>,
       alt!(alphanumeric | alt!(tag!("#t") | tag!("#f")))
);

named!(church_symbol<&str, char>,
       one_of!("!#$%&|*=-/:<=>?@^_~")
);

named!(church_primatives<&str, char>,
       one_of!("+-*/^%")
);

named!(church_parse_list<&str, Result<ChurchValue, ChurchParseError>>,
       do_parse!(
           begin: tag!("(") >>
           output: separated_list!(tag!(","), alphanumeric_or_bool) >>
           end: tag!(")") >>
           (ChurchValue::parse_vec_to_value(output))
       )
);
named!(church_parse_fn<&str, Result<ChurchValue, ChurchParseError>>,
       do_parse!(
           begin: opt!(tag!("(")) >>
           output: separated_list!(tag!(" "), take_while!(is_church_char)) >>
           end: opt!(tag!(")")) >>
           (ChurchValue::parse_vec_to_func(output))
       )
);
named!(church_parse<&str, Result<ChurchValue, ChurchParseError>>,
       alt!(alt!(alt!(complete!(parse_bool) | complete!(parse_number)) | complete!(church_parse_list)) | church_parse_fn)
);

pub fn read_expr(input: &str) -> Result<ChurchValue, ChurchParseError> {
    match church_parse(input) {
        IResult::Done(_, out) => Ok(out.unwrap()),
        IResult::Error(_) => Err(ChurchParseError::ParseError),
        _ => Err(ChurchParseError::ParseError),
    }
}
