/*!
## Problem 5. Smallest Multiple

<p>$2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.</p>
<p>What is the smallest positive number that is <strong class="tooltip">evenly divisible<span class="tooltiptext">divisible with no remainder</span></strong> by all of the numbers from $1$ to $20$?</p>
*/

pub fn answer() -> u64 {
    let n = 2520;
    let must_by_divisible_by = vec![
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20
    ];

    let mut i = 100;
    while true {
        let mut found = true;
        for divide_by in &must_by_divisible_by {
            if i % divide_by != 0 {
                i += 1;
                found = false;
            }
        }

        if found {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(), 232792560);
    }
}
