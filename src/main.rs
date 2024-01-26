mod algebra;
mod parser;

fn main() {
    let expr = parser::parse(&std::env::args().nth(1).unwrap());
    println!("{}", expr.string());
    println!("{}", expr.latex());
    println!("{:.20}", expr.eval(&std::collections::HashMap::new()));
}
