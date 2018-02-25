#[macro_use]
extern crate nom;

use church::parser::read_expr;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok().expect("FAILED");
    let res = read_expr(&input);
    println!("{}\n\n\n", res.to_string());
}
