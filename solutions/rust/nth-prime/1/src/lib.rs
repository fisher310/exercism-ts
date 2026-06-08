pub fn nth(n: u32) -> u32 {
    let mut k = 0_u32;
    let mut i = 2_u32;
    loop {
        if is_prime(i) {
            if k == n {
                return i
            }
            k += 1;
        }
        i += 1;
    }
}

fn is_prime(n :u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false
        }
    }
    true
}