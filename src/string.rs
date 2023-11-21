//! 文字列関連のライブラリです  
pub fn rotate(s: String) -> bool {
    /// 回文判定をします  
    let q = s.chars().collect::<Vec<char>>();
    let n = q.len();
    for i in 0..(n / 2) {
        if q[i] != q[n - i - 1] {
            return false;
        }
    }
    true
}

pub fn rotate_diff(q: String) -> usize {
    /// 回文にするために変更しなければいけない文字数を返します  
    let s = q.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut result: usize = 0;
    for i in 0..(n / 2) {
        if s[i] != s[n - i - 1] {
            result += 1;
        }
    }
    result
}

pub fn rle(s: String) -> Vec<(char, usize)> {
    let q = s.chars().collect::<Vec<char>>();
    let mut v: Vec<(char, usize)> = vec![];
    let mut now: char = q[0];
    let mut streek: usize = 1;
    for i in 1usize..q.len() {
        if now != q[i] {
            v.push((now, streek));
            now = q[i];
            streek = 1;
        } else {
            streek += 1;
        }
    }
    v.push((now, streek));
    v
}
