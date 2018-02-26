#![feature(const_fn)]

#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;

#[allow(unused_imports)]
#[macro_use]
extern crate nom;

#[allow(unused_imports)]
#[macro_use]
extern crate quick-error;

mod church;
use church::parser::read_expr;
use church::parser::*;

fn main() {
    let mut input = String::new();
    match apply("-", vec![ChurchValue::Number(2), ChurchValue::Number(2)]) {
        Ok(_) => println!("Ok"),
        Err(err) => println!("{:?}", err),
    }
    std::io::stdin().read_line(&mut input).ok().expect("FAILED");
    let res = read_expr(&input);
    println!("{}\n\n\n", res.to_string());
}
