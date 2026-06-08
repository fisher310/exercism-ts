pub fn encode(n: u64) -> String {
    insert_scale(breaking(n).iter().map(|x| say(*x)).collect::<Vec<String>>())
}

fn say(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        21..=29 => "twenty-".to_string() + &say(n - 20),
        30 => "thirty".to_string(),
        31..=39 => "thirty-".to_string() + &say(n - 30),
        40 => "forty".to_string(),
        41..=49 => "forty-".to_string() + &say(n - 40),
        50 => "fifty".to_string(),
        51..=59 => "fifty-".to_string() + &say(n - 50),
        60 => "sixty".to_string(),
        61..=69 => "sixty-".to_string() + &say(n - 60),
        70 => "seventy".to_string(),
        71..=79 => "seventy-".to_string() + &say(n - 70),
        80 => "eighty".to_string(),
        81..=89 => "eighty-".to_string() + &say(n - 80),
        90 => "ninety".to_string(),
        91..=99 => "ninety-".to_string() + &say(n - 90),
        100..=999 => {
            let mut h = say(n / 100) + " hundred";
            let x = n % 100;
            if x > 0 {
                h = h + " " + &say(x);
            }
            h
        }
        _ => panic!("the say value can't be more than 999."),
    }
}

fn breaking(mut n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    if n == 0 {
        res.push(0);
        return res;
    }

    while n > 0 {
        res.push(n % 1000);
        n /= 1000;
    }

    res
}

fn insert_scale(values: Vec<String>) -> String {
    let size = values.len();
    values
        .into_iter()
        .enumerate()
        .map(|(idx, mut value)| match idx {
            0 if size > 0 && value == "zero" => "".to_string(),
            0 => value.to_string(),
            _ if "zero" == value => "".to_string(),
            1 => {
                value.push_str(" thousand");
                value.to_string()
            }
            2 => {
                value.push_str(" million");
                value.to_string()
            }
            3 => {
                value.push_str(" billion");
                value.to_string()
            }
            4 => {
                value.push_str(" trillion");
                value.to_string()
            }
            5 => {
                value.push_str(" quadrillion");
                value.to_string()
            }
            6 => {
                value.push_str(" quintillion");
                value.to_string()
            }
            7 => {
                value.push_str(" sextillion");
                value.to_string()
            }
            _ => panic!("too large."),
        })
        .filter(|s| !s.is_empty())
        .rev()
        .collect::<Vec<String>>()
        .join(" ")
}
