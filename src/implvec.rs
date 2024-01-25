use itertools::Itertools;
use std::fmt::{Display, Formatter, Result};

#[allow(non_camel_case_types)]
pub struct Implvec(pub Vec<usize>);

#[allow(dead_code)]
impl Implvec {
    fn sort(&mut self) {
        self.0.sort();
    }
    fn binary_search(&self, n: &usize) -> Result<usize, usize> {
        self.0.binary_search(n)
    }
}

impl Display for Implvec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0.iter().join(" "))
    }
}
