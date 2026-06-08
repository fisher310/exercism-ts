pub fn series(digits: &str, len: usize) -> Vec<String> {
    let s = digits.chars().collect::<Vec<char>>();
    let n = s.len();

    let mut res = Vec::new();
    if n < len {
        return res;
    }
    for i in 0..=(n - len) {
        let mut sub = String::new();
        for j in &s[i..i + len] {
            sub.push(*j);
        }

        res.push(sub);
    }

    res
}
