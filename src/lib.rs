pub mod extended_int;
pub mod implvec;
pub mod inter;
pub mod matrix;
pub mod modint;
pub mod string;
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use crate::extended_int::ExtendedInt;
    use crate::implvec::Implvec;
    use crate::inter::{power, powm, rt};
    use crate::matrix::matrix_pow;
    use crate::modint::Modint;
    use crate::string::{rle, rotate, rotate_diff};
    use itertools::Itertools;
    #[test]
    fn powtest() {
        let result = power(4, 6);
        assert_eq!(result, 4096);
    }
    #[test]
    fn powmod() {
        let result = powm(123456789, 6574837563712, 234567894);
        assert_eq!(result.fact, 120678297);
        let result = powm(12, 7, 15);
        assert_eq!(result.fact, 3);
        let result = powm(73251, 73251, 998244353);
        assert_eq!(result.fact, 21540034);
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
    #[test]
    fn rotation() {
        assert!(rotate("aaaaa".to_string()));
        assert!(!rotate("abcdeecba".to_string()));
        assert!(rotate("abcdedcba".to_string()));
        assert_eq!(0, rotate_diff("abcdedcba".to_string()));
        assert_eq!(4, rotate_diff("dcbaedcba".to_string()));
    }
    #[test]
    fn rle_test() {
        assert_eq!(rle("ABC".to_string()), vec![('A', 1), ('B', 1), ('C', 1)]);
        assert_eq!(rle("AABCC".to_string()), vec![('A', 2), ('B', 1), ('C', 2)]);
    }
    #[test]
    fn power_matrix() {
        let k = matrix_pow(
            vec![vec![1000000000, 1], vec![0, 1]],
            998244353,
            1000000000000,
        )[0][1]
            .fact;
        assert_eq!(k, 919667211);
    }
    #[test]
    fn extended_pow() {
        let k = ExtendedInt::new(4usize);
        let r = ExtendedInt::new(3usize);
        let v = Implvec(vec![1usize, 2, 3]);
        println!("{}", v);
        assert_eq!(format!("{}", v), "1 2 3".to_string());
        assert_eq!((r ^ k).unwrap(), 81usize);
        assert_eq!(2, 2);
    }
    #[test]
    fn test_implvec() {
        let v = Implvec(vec![1usize, 2, 3]);
        assert_eq!(format!("{}", v), "1 2 3".to_string());
    }
}
