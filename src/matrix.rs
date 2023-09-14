//! 行列関連のライブラリです

use crate::modint::Modint;
/// 行列累乗です  
/// 行列`S`の`x`乗を`O(logx)`で計算します  
pub fn matrix_pow(s: Vec<Vec<usize>>, m: usize, mut x: usize) -> Vec<Vec<Modint>> {
    let mut r: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0); s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            r[i][j] = Modint::new(m, s[i][j]);
        }
    }
    let mut v: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0usize); r.len()]; r.len()];
    for i in 0..r.len() {
        v[i][i] = Modint::new(m, 1usize);
    }
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            let mut d: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0usize); r.len()]; r.len()];
            for i in 0..r.len() {
                for j in 0..r.len() {
                    for k in 0..r.len() {
                        d[i][j] += v[i][k] * r[k][j];
                    }
                }
            }
            x ^= 1usize << i;
            v = d;
        }
        let mut d: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0usize); r.len()]; r.len()];
        for i in 0..r.len() {
            for k in 0..r.len() {
                for j in 0..r.len() {
                    d[i][j] += r[i][k] * r[k][j];
                }
            }
        }
        r = d;
        i += 1;
    }
    v
}
