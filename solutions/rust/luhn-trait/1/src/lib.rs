pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut tmp = String::new();
        for ch in self.to_string().chars() {
            if ch == ' ' {
                continue;
            } else if ch.is_ascii_digit() {
                tmp.push(ch);
            } else {
                return false;
            }
        }

        if tmp.len() < 2 {
            return false;
        }

        tmp.chars()
            .rev()
            .enumerate()
            .map(|(idx, ch)| {
                let value = ch.to_digit(10).unwrap();
                if idx % 2 == 0 {
                    value
                } else {
                    let value = value * 2;
                    if value > 9 {
                        value - 9
                    } else {
                        value
                    }
                }
            })
            .sum::<u32>()
            % 10
            == 0
    }
}
