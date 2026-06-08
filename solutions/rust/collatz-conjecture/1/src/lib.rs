pub fn collatz(mut n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    let mut res = 0;

    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        res += 1;
    }

    Some(res)

}
