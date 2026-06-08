pub fn encode(source: &str) -> String {
    let mut res = String::new();
    let mut count = 0;
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        count += 1;
        if chars.peek() != Some(&ch) {
            if count > 1 {
                res.push_str(&count.to_string());
            }
            res.push(ch);
            count = 0;
        }
    }

    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let mut count = 0;

    for ch in source.chars() {
        if ch.is_ascii_digit() {
            count = count * 10 + ch.to_digit(10).unwrap()
        } else if count > 0 {
            for _ in 1..=count {
                res.push(ch);
            }
            count = 0;
        } else {
            res.push(ch);
        }
    }

    res
}
