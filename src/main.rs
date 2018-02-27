#![feature(const_fn)]

#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;

#[allow(unused_imports)]
#[macro_use]
extern crate nom;

#[allow(unused_imports)]
#[macro_use]
extern crate quick_error;

mod church;
use church::parser::read_expr;
use church::parser::*;

fn main() {
    let mut input = String::new();
    /*match apply("-", vec![apply("^", vec![ChurchValue::Number(2), ChurchValue::Number(13)]).unwrap(), ChurchValue::Number(1)]) {
        Ok(val) => println!("{}", val.to_string()),
        Err(err) => println!("{}", err),
}*/
    std::io::stdin().read_line(&mut input).ok().expect("FAILED");
    let res = read_expr(&input);
    println!("{}\n\n\n", eval(res.unwrap()).unwrap().to_string());
}
