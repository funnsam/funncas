use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableOrder {
    pub variable: String,
    pub order: RawHash<f64>,
}

pub struct Coefficients(pub HashMap<Vec<VariableOrder>, f64>);

#[derive(Debug, Clone, PartialEq)]
pub struct RawHash<T>(pub T);
impl<T: Default + Clone + PartialEq + Sized> std::cmp::Eq for RawHash<T> {}

impl<T: Default + Clone + PartialEq + Sized> std::hash::Hash for RawHash<T> {
    fn hash<H: std::hash::Hasher>(&self, hash: &mut H) {
        hash.write(unsafe { std::slice::from_raw_parts(&self.0 as *const T as *const u8, core::mem::size_of::<T>()) });
    }
}

use std::fmt::*;
impl Display for Coefficients {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (v, c) in self.0.iter() {
            writeln!(f, "{c}({})", v
                .iter()
                .map(|VariableOrder { variable, order }| format!("{variable}^{}", order.0))
                .collect::<Vec<String>>()
                .join(" + ")
            )?;
        }

        Ok(())
    }
}
