/*!
## Problem 4. Largest Palindrome Product

<p>A palindromic number reads the same both ways. The largest palindrome made from the product of two $2$-digit numbers is $9009 = 91 \times 99$.</p>
<p>Find the largest palindrome made from the product of two $3$-digit numbers.</p>
*/
use crate::math;

pub fn answer() -> u64 {
    let mut largest_product = 0;

    for i in (1..1000).rev() {
        for j in (1..1000).rev() {
            let product = i * j;
            if product > largest_product && math::is_palindromic(product) {
                largest_product = product;
            }
        }
    }

    largest_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(), 906609);
    }
}
