mod inter {
    pub fn powm(n: usize, m: usize, c: usize) -> usize {
        let k: usize = 1;
        let mut x = c;
        let mut b: usize = n;
        let mut a: usize = 1;
        let mut i = 0;
        while x != 0 {
            if k << i & x != 0 {
                a = (a * b) % m;
                b = (b * b) % m;
                x ^= k << i;
            } else {
                b = (b * b) % m;
            }
            i += 1;
        }
        a
    }
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
    pub fn rt(n: usize) -> usize {
        let mut l: usize = 1;
        let mut r: usize = n;
        let mut m: usize = 0;
        while r - l > 1 {
            m = (r + l) / 2;
            if m * m == n {
                return m;
            }
            if m * m > n {
                r = m;
            } else {
                l = m;
            }
        }
        return l;
    }
}
#[cfg(test)]
mod tests {
    use super::inter::*;
    #[test]
    fn powtest() {
        let result = power(4, 6);
        assert_eq!(result, 4096);
    }
    #[test]
    fn powmod() {
        let result = powm(123456789, 234567894, 6574837563712);
        assert_eq!(result, 120678297);
        let result = powm(12, 15, 7);
        assert_eq!(result, 3);
    }
    #[test]
    fn rttest() {
        let result = rt(4);
        assert_eq!(result, 2);
        let result = rt(100);
        assert_eq!(result, 10);
        let result = rt(1000);
        assert_eq!(result, 31);
        let result = rt(9999);
        assert_eq!(result, 99);
    }
}
