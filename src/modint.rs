//! 俗にいう一般的なModintです
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[allow(non_snake_case)]
pub struct Modint {
    /// どの値でmod を取るかの情報が入っています。
    pub MOD: usize,
    /// ここにModintの実体が入っています。結果を取り出すときはこの値を取り出すことになります  
    pub fact: usize,
}

impl Modint {
    /// first_modに剰余を取る値、initに初期値を渡してください  
    pub fn new(first_mod: usize, init: usize) -> Modint {
        Modint {
            MOD: first_mod,
            fact: init,
        }
    }
}

impl ops::Add for Modint {
    type Output = Modint;
    fn add(self, other: Self) -> Self {
        Modint::new(self.MOD, (self.fact + other.fact) % self.MOD)
    }
}
impl ops::Mul for Modint {
    type Output = Modint;
    fn mul(self, other: Self) -> Self {
        Modint::new(self.MOD, (self.fact * other.fact) % self.MOD)
    }
}
impl ops::Div for Modint {
    type Output = Modint;
    fn div(self, other: Self) -> Self {
        Modint::new(self.MOD, self.fact / other.fact)
    }
}
impl ops::Sub for Modint {
    type Output = Modint;
    fn sub(self, other: Self) -> Self {
        Modint::new(self.MOD, (self.MOD + self.fact + other.fact) % self.MOD)
    }
}
impl ops::AddAssign for Modint {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl ops::SubAssign for Modint {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl ops::MulAssign for Modint {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
