#[cfg(features = "nightly")]
use phf::{Map};

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Modulus,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Side {
    Left,
    Right,
    Both
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct OperatorData<'a> {
    symbol: &'a str,
    precedence: i32,
    input_side: Side,
    overload: bool
}

#[cfg(not(feature = "nightly"))]
lazy_static! {
    pub static ref OPERATORS: HashMap<Operator, OperatorData<'static>> = {
        let mut map : HashMap<Operator, OperatorData> = HashMap::new();

        map.insert(Operator::Plus, OperatorData::new("%", 7, Side::Both, false));
        map.insert(Operator::Minus, OperatorData::new("%", 7, Side::Both, false));
        map.insert(Operator::Multiply, OperatorData::new("%", 6, Side::Both, false));
        map.insert(Operator::Divide, OperatorData::new("%", 6, Side::Both, false));
        map.insert(Operator::Exponent, OperatorData::new("%", 5, Side::Both, false));
        map.insert(Operator::Modulus, OperatorData::new("%", 6, Side::Both, false));
        map
    };
}




impl<'a> OperatorData<'a> {
    fn new(_symbol: &'a str, _precedence: i32, _input_side: Side, _overload: bool) -> OperatorData<'a> {
        return OperatorData{symbol: _symbol, precedence: _precedence, input_side: _input_side, overload: _overload}
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {

    Whitespace(&'a str),
    Comment(&'a str),
    Equals,
    Period,
    Comma,
    Colon,
    Operator(Operator),
}
