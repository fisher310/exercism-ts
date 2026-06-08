pub fn factors(n: u64) -> Vec<u64> {
    let mut ans = Vec::new();
    if n == 1 {
        return vec![];
    }
    let mut x = 2;
    let mut y = n;

    while x <= y {
        if y % x == 0 {
            ans.push(x);
            y /= x;
        } else {
            x += 1;
        }
    }

    ans
}