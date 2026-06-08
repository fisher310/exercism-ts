use rand::Rng;
pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // g^a mod p
    fast_power(g as u128, a as u128, p as u128)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // b_pub ^ a mod p

    fast_power(b_pub as u128, a as u128, p as u128)
}


fn fast_power(mut a : u128, mut b: u128, m : u128) -> u64 {
    a %= m;

    let mut res = 1;

    while b > 0 {
        if b & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }

    res as u64
}
