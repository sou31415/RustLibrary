pub fn rt(n: usize) -> usize {
    let mut l: u128 = 1;
    let mut r: u128 = n as u128;
    let mut m: u128 = 0;
    while r - l > 1 {
        m = (r + l) / 2;
        if m * m == n as u128 {
            return m as usize;
        }
        if m * m > n as u128 {
            r = m;
        } else {
            l = m;
        }
    }
    return l as usize;
}
