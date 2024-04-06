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
