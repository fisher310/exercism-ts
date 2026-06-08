/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if value % 10 == 0 {
            return None;
        }

        let mut reverse = 0;
        let mut curr = value;
        while curr > 0 {
            reverse = reverse * 10 + curr % 10;
            curr /= 10;
        }

        if reverse == value {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p: Option<Palindrome> = None;
    let mut max_p: Option<Palindrome> = None;
    for i in min..=max {
        for j in min..=max {
            let k = i * j;
            if let Some(p) = Palindrome::new(k) {
                if let Some(m) = min_p {
                    if p.0 < m.0 {
                        min_p = Some(p)
                    }
                } else {
                    min_p = Some(p);
                }

                if let Some(x) = max_p {
                    if p.0 > x.0 {
                        max_p = Some(p);
                    }
                } else {
                    max_p = Some(p);
                }
            }
        }
    }

    Some((min_p?, max_p?))
}
