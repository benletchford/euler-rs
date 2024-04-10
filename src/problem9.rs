/*!
## Problem 9. Special Pythagorean Triplet

<p>A Pythagorean triplet is a set of three natural numbers, $a \lt b \lt c$, for which,
$$a^2 + b^2 = c^2.$$</p>
<p>For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.</p>
<p>There exists exactly one Pythagorean triplet for which $a + b + c = 1000$.<br>Find the product $abc$.</p>
*/
use crate::math;

pub fn answer() -> u32 {
    let find = 1000;

    for a in 1..1000 {
        let af32 = a as f32;
        for b in 1..1000 {
            let bf32 = b as f32;
            if !(af32 < bf32) {
                continue;
            }
            let cf32 = (af32 * af32 + bf32 * bf32).sqrt();

            if !(af32 < bf32 && bf32 < cf32) {
                continue;
            }

            if af32 + bf32 + cf32 == 1000.0 {
                return (af32 * bf32 * cf32) as u32;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(), 31875000);
    }
}
