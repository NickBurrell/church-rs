#[macro_use] extern crate lazy_static;

#[allow(unused_imports)]
#[macro_use]
extern crate nom;

#[allow(unused_imports)]
#[macro_use]
extern crate quick_error;

use std::collections::HashMap;

mod church;
use church::parser::read_expr;
use church::eval::*;

fn main() {
    let mut input = String::new();
    /*match apply("-", vec![apply("^", vec![ChurchValue::Number(2), ChurchValue::Number(13)]).unwrap(), ChurchValue::Number(1)]) {
        Ok(val) => println!("{}", val.to_string()),
        Err(err) => println!("{}", err),
}*/
    std::io::stdin().read_line(&mut input).ok().expect("FAILED");
    let res = read_expr(&input);
    let mut church_evaluator = Environment::new(HashMap::new(), HashMap::new());
    println!("{}", eval(&mut church_evaluator, res.unwrap()).unwrap().to_string());
}
