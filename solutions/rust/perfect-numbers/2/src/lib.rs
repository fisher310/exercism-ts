#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let mut sum = 0;
    for i in 1..num {
        if num % i == 0 {
            sum += i;
        }
    }

    match num.cmp(&sum) {
        std::cmp::Ordering::Less => Some(Classification::Abundant),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Deficient),
    }
}
