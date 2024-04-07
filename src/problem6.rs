/*!
## Problem 6. Sum Square Difference

<p>The sum of the squares of the first ten natural numbers is,</p>
$$1^2 + 2^2 + ... + 10^2 = 385.$$
<p>The square of the sum of the first ten natural numbers is,</p>
$$(1 + 2 + ... + 10)^2 = 55^2 = 3025.$$
<p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
<p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>
*/

pub fn answer() -> u64 {
    let mut sum_of_squares = 0;
    for i in 1..100 + 1 {
        sum_of_squares += i * i;
    }

    let mut square_of_sum = 0;
    for i in 1..100 + 1 {
        square_of_sum += i;
    }
    square_of_sum = square_of_sum * square_of_sum;
    println!("{}", square_of_sum);

    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(), 25164150);
    }
}
