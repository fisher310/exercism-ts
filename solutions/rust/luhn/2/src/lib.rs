/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut chs = Vec::new();
    for ch in code.chars() {
        if ch == ' ' {
            continue;
        } else if !ch.is_ascii_digit() {
            return false;
        } else {
            chs.push(ch.to_digit(10).unwrap());
        }
    }
    if chs.len() < 2 {
        return false;
    }

    chs.iter()
        .rev()
        .enumerate()
        .map(|(idx, value)| {
            if idx % 2 == 1 {
                let value = value * 2;
                if value > 9 {
                    value - 9
                } else {
                    value
                }
            } else {
                *value
            }
        })
        .sum::<u32>()
        % 10
        == 0
}
