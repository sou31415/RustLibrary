pub mod inter;
pub mod string;
#[cfg(test)]
mod tests {
    use crate::inter::{power, powm, rt};
    use crate::string::{rotate, rotate_diff};
    #[test]
    fn powtest() {
        let result = power(4, 6);
        assert_eq!(result, 4096);
    }
    #[test]
    fn powmod() {
        let result = powm(123456789, 6574837563712, 234567894);
        assert_eq!(result, 120678297);
        let result = powm(12, 7, 15);
        assert_eq!(result, 3);
        let result = powm(73251, 73251, 998244353);
        assert_eq!(result, 21540034);
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
}
