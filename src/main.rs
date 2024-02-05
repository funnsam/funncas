#![feature(generic_const_exprs)]

mod algebra;
mod parser;

fn main() {
    if std::env::args().count() != 2 {
        println!("\x1b[1;31mError:\x1b[0m 1 arg required only");
        std::process::exit(1);
    }

    let expr = parser::parse(&std::env::args().nth(1).unwrap());
    println!("String: {}", expr.string());
    println!("LaTeX:  {}", expr.latex());
    println!("Coefficients: {}", expr.coefficient());
    // println!("dx: {}", expr.d("x").string());
    let expr = expr.simplify();
    println!("Simplified: {}", expr.string());
    // println!("Evaluated:  {:.20}", expr.eval(&std::collections::HashMap::new()));
}
