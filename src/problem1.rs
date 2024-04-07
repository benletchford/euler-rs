/*!
## Problem 1. Multiples of 3 and 5

<p>If we list all the natural numbers below $10$ that are multiples of $3$ or $5$, we get $3, 5, 6$ and $9$. The sum of these multiples is $23$.</p>
<p>Find the sum of all the multiples of $3$ or $5$ below $1000$.</p>
*/

pub fn answer() -> i32 {
    let n = 1000;
    let mut running_sum = 0;
    for i in 3..n {
        if i % 3 == 0 || i % 5 == 0 {
            running_sum += i;
        }
    }

    running_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(), 233168)
    }
}
