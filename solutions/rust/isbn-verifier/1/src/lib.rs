/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars: Vec<char> = isbn.chars().filter(|c| c.is_digit(10)).collect();
    if chars.len() < 1 {
        return false;
    }

    let last = isbn.chars().last().unwrap();
    if (chars.len() == 10 && last != 'X') || (chars.len() == 9 && last == 'X') {
        let mut res = chars.iter().take(9).enumerate().fold(0, |sum, (idx, ch)| {
            sum + (10 - idx) as u32 * ch.to_digit(10).unwrap()
        });

        res = 11 - (res % 11);

        if res == 10 {
            return last == 'X';
        } else {
            if let Some(v) = last.to_digit(10) {
                return v == res;
            }
        }
    }

    false
}
