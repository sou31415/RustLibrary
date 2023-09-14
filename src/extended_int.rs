use std::ops;
#[allow(non_snake_case)]
#[derive(Clone, PartialEq, Eq, Copy)]
pub struct ExtendedInt {
    pub value: usize,
}

impl ExtendedInt {
    pub fn new(n: usize) -> ExtendedInt {
        ExtendedInt { value: n }
    }
    pub fn unwrap(&self) -> usize {
        self.value
    }
}

impl ops::BitXor for ExtendedInt {
    type Output = ExtendedInt;
    fn bitxor(self, other: Self) -> Self {
        let mut a: usize = 1;
        let mut b: usize = self.value;
        let mut i: usize = 1;
        let mut x = other.value;
        while x != 0 {
            if i & x != 0 {
                a *= b;
                b *= b;
                x ^= i;
            } else {
                b *= b;
            }
            i <<= 1;
        }
        ExtendedInt { value: a }
    }
}
