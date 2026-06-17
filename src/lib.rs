pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { (1..=n).product() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(5));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }
}
