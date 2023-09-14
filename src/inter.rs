//! 整数関連のライブラリ  
use crate::modint::Modint;
/// 繰り返し二乗法を用いて`n`の`x`乗を`O(logx)`で計算します  
pub fn power(n: usize, mut x: usize) -> usize {
    let mut a: usize = 1;
    let mut b: usize = n;
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            a *= b;
            b *= b;
            x -= 1usize << i;
        } else {
            b *= b;
        }
        i += 1;
    }
    a
}
/// 繰り返し二乗法を用いて`n^x mod m`を`O(logx)`で計算します  
pub fn powm(n: usize, mut x: usize, m: usize) -> Modint {
    let mut b = Modint::new(m, n);
    let mut a = Modint::new(m, 1);
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            a *= b;
            b *= b;
            x ^= 1usize << i;
        } else {
            b *= b;
        }
        i += 1;
    }
    a
}

/// 二分探索を用いて`floor(sqrt(n))` を`O(logn)`で計算します
pub fn rt(n: usize) -> usize {
    let mut l: u128 = 1;
    let mut r: u128 = n as u128;
    while r - l > 1 {
        let m = (r + l) / 2;
        if m * m == n as u128 {
            return m as usize;
        }
        if m * m > n as u128 {
            r = m;
        } else {
            l = m;
        }
    }
    l as usize
}
