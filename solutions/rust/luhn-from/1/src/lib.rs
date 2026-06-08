pub struct Luhn {
    data: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut tmp = String::new();
        for ch in self.data.chars() {
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
                if idx % 2 == 1 {
                    let value = value * 2;
                    if value > 9 {
                        value - 9
                    } else {
                        value
                    }
                } else {
                    value
                }
            })
            .sum::<u32>()
            % 10
            == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
// impl<'a> From<&'a str> for Luhn {
//     fn from(input: &'a str) -> Self {
//         Luhn {
//             data: input.to_string(),
//         }
//     }
// }

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn {
            data: input.to_string(),
        }
    }
}
