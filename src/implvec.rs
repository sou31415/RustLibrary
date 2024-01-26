use itertools::Itertools;
use std::fmt::{self, Display, Formatter};

#[allow(non_camel_case_types)]
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Implvec(pub Vec<usize>);

#[allow(dead_code)]
impl Implvec {
    fn sort(&mut self) {
        self.0.sort();
    }
    fn binary_search(&self, n: &usize) -> Result<usize, usize> {
        self.0.binary_search(n)
    }
    fn push(&mut self, n: usize) {
        self.0.push(n);
    }
    fn pop(&mut self) -> Option<usize> {
        self.0.pop()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn dedup(&mut self) {
        self.0.dedup();
    }
}

impl Display for Implvec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().join(" "))
    }
}
impl From<Vec<usize>> for Implvec {
    fn from(item: Vec<usize>) -> Self {
        Self(item)
    }
}
