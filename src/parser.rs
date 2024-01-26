use crate::algebra::*;

pub fn parse(input: &str) -> Box<dyn ops::Expr> {
    let mut v: Vec<Box<dyn ops::Expr>> = Vec::new();
    for i in input.split_whitespace() {
        match i.chars().nth(0).unwrap() {
            '0'..='9' | '-' => v.push(Box::new(ops::Constant(i.parse().unwrap()))),
            '+' => {
                let nth = i[1..].parse().unwrap();
                let mut b = Vec::with_capacity(nth);
                for _ in 0..nth {
                    b.push(v.pop().unwrap());
                }

                b.reverse();
                v.push(Box::new(ops::Add(b)));
            },
            '*' => {
                let nth = i[1..].parse().unwrap();
                let mut b = Vec::with_capacity(nth);
                for _ in 0..nth {
                    b.push(v.pop().unwrap());
                }

                b.reverse();
                v.push(Box::new(ops::Mul(b)));
            },
            '^' => {
                let exp = v.pop().unwrap();
                let bas = v.pop().unwrap();
                v.push(Box::new(ops::Pow(bas, exp)));
            },
            _ => todo!(),
        }
    }

    v[0].clone()
}
