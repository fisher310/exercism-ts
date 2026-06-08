pub fn encode(source: &str) -> String {
    let mut res = String::new();
    let mut prev = None;
    let mut count = 0;

    for ch in source.chars() {
        if let Some(pc) = prev {
            if pc == ch {
                count += 1;
            } else {
                if count > 1 {
                    res.push_str(&count.to_string());
                }
                res.push(pc);
                prev = Some(ch);
                count = 1;
            }
        } else {
            prev = Some(ch);
            count = 1;
        }
    }

    if count > 0 {
        if count > 1 {
            res.push_str(&count.to_string());
        }
        res.push(prev.unwrap());
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
