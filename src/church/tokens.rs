use std::str::FromStr;
use std::clone::Clone;

use super::error::*;
use super::utils::*;
use super::primatives::{ChurchValue};

 
#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Modulus,
}

///
///
///
#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {

    Whitespace(&'a str),
    Comment(&'a str),
    Equals,
    Period,
    Comma,
    Colon,
    Operator(Op),
    Value(Box<ChurchValue>),
}
