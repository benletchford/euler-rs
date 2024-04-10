/*!
## Problem 10. Summation of Primes

<p>The sum of the primes below $10$ is $2 + 3 + 5 + 7 = 17$.</p>
<p>Find the sum of all the primes below two million.</p>
*/
use crate::math;

pub fn answer() -> u64 {
    let less_than = 2000000;

    let mut sum_of_primes: u64 = 0;
    for i in 1..less_than {
        if math::is_prime(i) {
            sum_of_primes += i;
        }
    }

    sum_of_primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_answer() {
        assert_eq!(answer(), 142913828922);
    }
}
