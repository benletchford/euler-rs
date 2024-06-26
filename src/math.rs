pub fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn is_palindromic(n: u64) -> bool {
    let n_str = n.to_string();

    n_str == n_str.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(13), true);
    }

    #[test]
    fn test_is_palindromic() {
        assert_eq!(is_palindromic(1), true);
        assert_eq!(is_palindromic(2345), false);
        assert_eq!(is_palindromic(90909), true);
        assert_eq!(is_palindromic(90908), false);
        assert_eq!(is_palindromic(123454321), true);
    }
}
