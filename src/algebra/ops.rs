use std::collections::HashMap;

pub type Arguments = HashMap<String, f64>;

pub trait Expr {
    fn string(&self) -> String;
    fn latex(&self) -> String;
    fn eval(&self, _: &Arguments) -> f64;
    fn d(&self, _: &String) -> Box<dyn Expr>;

    fn clone_dyn(&self) -> Box<dyn Expr>;
    fn get_const(&self) -> Option<f64>;
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

    fn d(&self, _: &String) -> Box<dyn Expr> {
        Box::new(Constant(0.0))
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
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

    fn d(&self, n: &String) -> Box<dyn Expr> {
        if *n == self.0 {
            Box::new(Constant(1.0))
        } else {
            Box::new(Constant(0.0))
        }
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
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

    fn d(&self, n: &String) -> Box<dyn Expr> {
        let mut d = Vec::with_capacity(self.0.len());

        for i in self.0.iter() {
            d.push(i.d(n))
        }

        Box::new(Add(d))
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

    fn d(&self, n: &String) -> Box<dyn Expr> {
        let mut terms: Vec<Box<dyn Expr>> = Vec::with_capacity(self.0.len());

        for i in 0..self.0.len() {
            let mut p = self.0.clone();
            p[i] = p[i].d(n);
            terms.push(Box::new(Mul(p)));
        }

        Box::new(Add(terms))
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

    fn d(&self, n: &String) -> Box<dyn Expr> {
        if let Some(exp) = self.1.get_const() {
            Box::new(Mul(vec![self.1.clone(), Box::new(Pow(self.0.clone(), Box::new(Constant(exp-1.0)))), self.0.d(n)]))
        } else {
            todo!()
        }
    }

    fn clone_dyn(&self) -> Box<dyn Expr> {
        Box::new(self.clone())
    }

    fn get_const(&self) -> Option<f64> { None }
}
