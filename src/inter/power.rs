pub fn power(n: usize, r: usize) -> usize {
    let k: usize = 1;
    let mut x: usize = r;
    let mut a: usize = 1;
    let mut b: usize = n;
    let mut i: usize = 0;
    while x != 0 {
        if k << i & x != 0 {
            a *= b;
            b *= b;
            x -= k << i;
        } else {
            b *= b;
        }
        i += 1;
    }
    a
}
