/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut chs = code.chars().filter(|c| *c != ' ').collect::<Vec<char>>();
    if chs.is_empty() || chs.len() < 2 {
        return false;
    }
    chs.reverse();
    let len = chs.len();
    let mut idx = 1;

    while idx < len {
        let ch = chs[idx];
        if ch < '0' || ch > '9' {
            return false;
        }
        let value = ch as u32 - '0' as u32;
        let value = value * 2;
        let value = if value > 9 { value - 9 } else { value };
        chs[idx] = char::from_digit(value, 10).unwrap();
        idx += 2;
    }

    println!("{:?}", chs);
    let mut sum = 0;
    for ch in chs {
        if ch < '0' || ch > '9' {
            return false
        }
        sum += ch as u32 - '0' as u32;
    }


    sum % 10 == 0
}
