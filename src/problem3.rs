/*!
## Problem 3. Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?
*/

use crate::math;

pub fn answer() -> u64 {
    let a = 600851475143;

    for i in a / 2..a {
        let j = a - i;
        if a % j == 0 && math::is_prime(j) {
            return j;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_answer() {
        assert_eq!(answer(), 6857)
    }
}
