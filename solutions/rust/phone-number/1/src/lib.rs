pub fn number(user_number: &str) -> Option<String> {
    let mut clean_number: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();

    if clean_number.starts_with('1') {
        if clean_number.len() != 11 {
            return None;
        } else {
            clean_number.remove(0);
        }
    }
    if clean_number.len() != 10 {
        return None;
    }

    for (idx, n) in clean_number.chars().enumerate() {
        match idx {
            0 | 3 => {
                let x = n.to_digit(10).unwrap();
                if x <= 1 {
                    return None;
                }
            }
            1..=2 => {}
            _ => {
                break;
            }
        }
    }

    Some(clean_number)
}
