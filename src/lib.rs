pub mod inter;
#[cfg(test)]
mod tests {
    use crate::inter::{power::power, powm::powm, rt::rt};
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
