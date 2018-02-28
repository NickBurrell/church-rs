use phf::{Map};

use std::collections::HashMap;

use super::error::*;
use super::utils::*;
use super::parser::*;

pub mod primatives {
    use super::*;
    use super::super::error::*;
    fn add(v1: ChurchValue, v2: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match v1 {
            ChurchValue::Number(x) => {
                match v2 {
                    ChurchValue::Number(y) => {
                        Ok(ChurchValue::Number(x+y))
                    },
                    _ => Err(ChurchEvalError::TypeError(String::from("+"), String::from(""), String::from("")))
                }
            },
            _ => Err(ChurchEvalError::TypeError(String::from("+"), String::from(""), String::from("")))
        }
    }
    fn sub(v1: ChurchValue, v2: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match v1 {
            ChurchValue::Number(x) => {
                match v2 {
                    ChurchValue::Number(y) => {
                        Ok(ChurchValue::Number(x-y))
                    },
                    _ => Err(ChurchEvalError::TypeError(String::from("-"), String::from(""), String::from("")))
                }
            },
            _ => Err(ChurchEvalError::TypeError(String::from("-"), String::from(""), String::from("")))
        }
    }
    fn mul(v1: ChurchValue, v2: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match v1 {
            ChurchValue::Number(x) => {
                match v2 {
                    ChurchValue::Number(y) => {
                        Ok(ChurchValue::Number(x*y))
                    },
                    _ => Err(ChurchEvalError::TypeError(String::from("*"), String::from(""), String::from("")))
                }
            },
            _ => Err(ChurchEvalError::TypeError(String::from("*"), String::from(""), String::from("")))
        }
    }
    fn div(v1: ChurchValue, v2: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match v1 {
            ChurchValue::Number(x) => {
                match v2 {
                    ChurchValue::Number(y) => {
                        Ok(ChurchValue::Number(x/y))
                    },
                    _ => Err(ChurchEvalError::TypeError(String::from("/"), String::from(""), String::from("")))
                }
            },
            _ => Err(ChurchEvalError::TypeError(String::from("/"), String::from(""), String::from("")))
        }
    }
    fn exp<'a>(v1: ChurchValue, v2: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match v1 {
            ChurchValue::Number(x) => {
                match v2 {
                    ChurchValue::Number(y) => {
                        Ok(ChurchValue::Number(x.pow(y as u32)))
                    },
                    _ => Err(ChurchEvalError::TypeError(String::from("+"), String::from(""), String::from("")))
                }
            },
            _ => Err(ChurchEvalError::TypeError(String::from("+"), String::from(""), String::from("")))
        }
    }
    fn modu<'a>(v1: ChurchValue, v2: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match v1 {
            ChurchValue::Number(x) => {
                match v2 {
                    ChurchValue::Number(y) => {
                        Ok(ChurchValue::Number(x%y))
                    },
                    _ => Err(ChurchEvalError::TypeError(String::from("+"), String::from(""), String::from("")))
                }
            },
            _ => Err(ChurchEvalError::TypeError(String::from("+"), String::from(""), String::from("")))
        }
    }
    pub static PRIMATIVES: Map<&'static str, fn(ChurchValue, ChurchValue) -> Result<ChurchValue, ChurchEvalError>> = phf_map! {
        "+" => add,
        "-" => sub,
        "*" => mul,
        "/" => div,
        "^" => exp,
        "%" => modu,
    };
}

pub struct Evaluator {
    vars: HashMap<String, ChurchValue>,
    func: HashMap<String, ChurchValue>,
}

impl Evaluator {

    pub fn eval(&self, input: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match input {
            ChurchValue::List(data) => Ok(ChurchValue::List(data)),
            ChurchValue::Bool(data) => Ok(ChurchValue::Bool(data)),
            ChurchValue::Number(data) => Ok(ChurchValue::Number(data)),
            ChurchValue::Func(fn_name, args) => {
                self.apply(&fn_name, *args)
            }
        }
    }
    pub fn eval_statement(input: ChurchValue) -> Result<ChurchValue, ChurchEvalError> {
        match input {
            ChurchValue::List(data) => Ok(ChurchValue::List(data)),
            ChurchValue::Bool(data) => Ok(ChurchValue::Bool(data)),
            ChurchValue::Number(data) => Ok(ChurchValue::Number(data)),
            ChurchValue::Func(fn_name, args) => {
                Evaluator::apply_statement(&fn_name, *args)
            }
        }
    }

    pub fn apply(&self, fn_name: &str, args: Vec<ChurchValue>) -> Result<ChurchValue, ChurchEvalError> {
        let function = primatives::PRIMATIVES.get(fn_name);
        match function {
            Some(fun) => {
                if args.len() < 2 {
                    return Err(ChurchEvalError::ArgumentError(fn_name.to_owned(), Box::new(args)))
                }
                let arg1 = args[0].clone();
                let arg2 = args[1].clone();
                let out = fun(arg1, arg2);
                out
            },
            None => {
                Err(ChurchEvalError::FunctionNotFound(fn_name.to_owned()))
            }

        }

    }

    pub fn apply_statement(fn_name: &str, args: Vec<ChurchValue>) -> Result<ChurchValue, ChurchEvalError> {
        let function = primatives::PRIMATIVES.get(fn_name);
        match function {
            Some(fun) => {
                if args.len() < 2 {
                    return Err(ChurchEvalError::ArgumentError(fn_name.to_owned(), Box::new(args)))
                }
                let arg1 = args[0].clone();
                let arg2 = args[1].clone();
                let out = fun(arg1, arg2);
                out
            },
            None => {
                Err(ChurchEvalError::FunctionNotFound(fn_name.to_owned()))
            }

        }

    }
}
