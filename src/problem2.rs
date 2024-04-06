pub fn answer() -> i32 {
    let mut a = 1;
    let mut b = 2;

    let mut running_even_sum = b;
    let mut ab = 0;
    while ab < 4000000 {
        ab = a + b;
        a = b;
        b = ab;

        if ab % 2 == 0 {
            running_even_sum += ab;
        }
    }

    running_even_sum
}
