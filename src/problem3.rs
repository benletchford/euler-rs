use crate::math;

pub fn answer() -> u64 {
    let a = 600851475143;

    let mut highest_prime = 0;
    for i in a / 2..a {
        let j = a - i;
        if a % j == 0 && math::is_prime(j) {
            return j;
        }
    }

    0
}
