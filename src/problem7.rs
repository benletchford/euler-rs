/*!
## Problem 7. 10001st Prime

<p>By listing the first six prime numbers: $2, 3, 5, 7, 11$, and $13$, we can see that the $6$th prime is $13$.</p>
<p>What is the $10\,001$st prime number?</p>
*/
use crate::math;

pub fn answer() -> u64 {
    let nth_prime = 10001;

    let mut current_nth_prime = 0;
    let mut current_prime = 0;
    let mut n = 2;
    loop {
        if math::is_prime(n) {
            current_nth_prime += 1;
            current_prime = n;
        }

        if current_nth_prime == nth_prime {
            return current_prime;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(), 104743);
    }
}
