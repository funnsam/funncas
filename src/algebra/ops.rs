use std::collections::HashMap;
use super::coeff::*;

pub type Arguments = HashMap<String, f64>;

pub trait Expr {
    fn string(&self) -> String;
    fn latex(&self) -> String;
    fn eval(&self, _: &Arguments) -> f64;
    fn d(&self, _: &str) -> Box<dyn Expr>;

    fn simplify(&self) -> Box<dyn Expr>;

    fn clone_dyn(&self) -> Box<dyn Expr>;
    fn coefficient(&self) -> Coefficients;

    fn get_const(&self) -> Option<f64> {
        self.coefficient().0.get(&vec![]).copied()
    }
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

#[derive(Clone)]
pub struct Constant(pub f64);

impl Expr for Constant {
    fn string(&self) -> String {
        if self.0.is_sign_positive() {
            self.0.to_string()
        } else {
            format!("({})", self.0)
        }
    }

    fn latex(&self) -> String {
        if self.0.is_sign_positive() {
            self.0.to_string()
        } else {
            format!("({})", self.0)
        }
    }

    fn eval(&self, _: &Arguments) -> f64 {
        self.0
    }

    fn d(&self, _: &str) -> Box<dyn Expr> {
        Box::new(Constant(0.0))
    }

    fn simplify(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn coefficient(&self) -> Coefficients {
        let mut coeff = HashMap::new();
        coeff.insert(vec![], self.0);
        Coefficients(coeff)
    }

    fn get_const(&self) -> Option<f64> {
        Some(self.0)
    }
}

#[derive(Clone)]
pub struct Variable(pub String);

impl Expr for Variable {
    fn string(&self) -> String {
        self.0.clone()
    }

    fn latex(&self) -> String {
        self.0.clone()
    }

    fn eval(&self, a: &Arguments) -> f64 {
        a[&self.0]
    }

    fn d(&self, n: &str) -> Box<dyn Expr> {
        if *n == self.0 {
            Box::new(Constant(1.0))
        } else {
            Box::new(Constant(0.0))
        }
    }

    fn simplify(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn coefficient(&self) -> Coefficients {
        let mut coeff = HashMap::new();
        coeff.insert(vec![VariableOrder {
            variable: self.0.clone(),
            order: RawHash(1.0)
        }], 1.0);
        Coefficients(coeff)
    }

    fn get_const(&self) -> Option<f64> { None }
}

#[derive(Clone)]
pub struct Add(pub Vec<Box<dyn Expr>>);

impl Expr for Add {
    fn string(&self) -> String {
        format!("({})", self.0.iter().map(|a| a.string()).collect::<Vec<String>>().join(" + "))
    }

    fn latex(&self) -> String {
        format!("({})", self.0.iter().map(|a| a.latex()).collect::<Vec<String>>().join(" + "))
    }

    fn eval(&self, a: &Arguments) -> f64 {
        let mut sum = self.0[0].eval(a);
        for i in self.0.iter().skip(1) {
            sum += i.eval(a);
        }

        sum
    }

    fn d(&self, n: &str) -> Box<dyn Expr> {
        let mut d = Vec::with_capacity(self.0.len());

        for i in self.0.iter() {
            d.push(i.d(n))
        }

        Box::new(Add(d))
    }

    fn simplify(&self) -> Box<dyn Expr> {
        let mut simplified = Vec::with_capacity(self.0.len());
        let mut is_const = true;

        for i in self.0.iter() {
            let i = i.simplify();
            is_const &= i.get_const().is_some();
            simplified.push(i);
        }

        if is_const {
            Box::new(Constant(Self(simplified).eval(unsafe {
                core::mem::transmute(core::ptr::null::<u8>()) // casual unsafe action going on here
            })))
        } else {
            Box::new(Self(simplified))
        }
    }

    fn coefficient(&self) -> Coefficients {
        todo!()
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn get_const(&self) -> Option<f64> { None }
}

#[derive(Clone)]
pub struct Mul(pub Vec<Box<dyn Expr>>);

impl Expr for Mul {
    fn string(&self) -> String {
        format!("({})", self.0.iter().map(|a| a.string()).collect::<Vec<String>>().join(" × "))
    }

    fn latex(&self) -> String {
        format!("({})", self.0.iter().map(|a| a.latex()).collect::<Vec<String>>().join(r" \times "))
    }

    fn eval(&self, a: &Arguments) -> f64 {
        let mut sum = self.0[0].eval(a);
        for i in self.0.iter().skip(1) {
            sum *= i.eval(a);
        }

        sum
    }

    fn d(&self, n: &str) -> Box<dyn Expr> {
        let mut terms: Vec<Box<dyn Expr>> = Vec::with_capacity(self.0.len());

        for i in 0..self.0.len() {
            let mut p = self.0.clone();
            p[i] = p[i].d(n);
            terms.push(Box::new(Mul(p)));
        }

        Box::new(Add(terms))
    }

    // totally not copy-pasted
    fn simplify(&self) -> Box<dyn Expr> {
        let mut simplified = Vec::with_capacity(self.0.len());
        let mut is_const = true;

        for i in self.0.iter() {
            let i = i.simplify();
            is_const &= i.get_const().is_some();
            simplified.push(i);
        }

        if is_const {
            Box::new(Constant(Self(simplified).eval(unsafe {
                core::mem::transmute(core::ptr::null::<u8>()) // casual unsafe action going on here
            })))
        } else {
            Box::new(Self(simplified))
        }
    }

    fn coefficient(&self) -> Coefficients {
        todo!()
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn get_const(&self) -> Option<f64> { None }
}

#[derive(Clone)]
pub struct Pow(pub Box<dyn Expr>, pub Box<dyn Expr>);

impl Expr for Pow {
    fn string(&self) -> String {
        format!("({} ^ {})", self.0.string(), self.1.string())
    }

    fn latex(&self) -> String {
        format!(r"({}^{{{}}})", self.0.latex(), self.1.latex())
    }

    fn eval(&self, a: &Arguments) -> f64 {
        self.0.eval(a).powf(self.1.eval(a))
    }

    fn d(&self, n: &str) -> Box<dyn Expr> {
        if let Some(exp) = self.1.get_const() {
            Box::new(Mul(vec![
                self.1.clone(),
                Box::new(Pow(self.0.clone(), Box::new(Constant(exp-1.0)))),
                self.0.d(n)
            ]))
        } else {
            todo!()
        }
    }

    fn simplify(&self) -> Box<dyn Expr> {
        let lhs = self.0.simplify();
        let rhs = self.1.simplify();

        if lhs.get_const().is_some() && rhs.get_const().is_some() {
            Box::new(Constant(Self(lhs, rhs).eval(unsafe {
                core::mem::transmute(core::ptr::null::<u8>()) // casual unsafe action going on here
            })))
        } else {
            Box::new(self.clone())
        }
    }

    fn coefficient(&self) -> Coefficients {
        todo!()
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn get_const(&self) -> Option<f64> { None }
}
