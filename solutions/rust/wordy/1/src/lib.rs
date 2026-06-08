
pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") {
        return None;
    }

    let (n, rest) = num(&command[8..])?;
    op(n, rest)
}

fn op(n: i32, command: &str) -> Option<i32> {
    if command == "?" {
        Some(n)
    } else if command.starts_with(" plus ") {
        let (m, rest) = num(&command[6..])?;
        op(n + m, rest)
    } else if command.starts_with(" minus ") {
        let (m, rest) = num(&command[7..])?;
        op(n - m, rest)
    } else if command.starts_with(" multiplied by ") {
        let (m, rest) = num(&command[15..])?;
        op(n * m, rest)
    } else if command.starts_with(" divided by ") {
        let (m, rest) = num(&command[12..])?;
        if m == 0 {
            return None;
        }
        op(n / m, rest)
    } else if command.starts_with(" raised to the ") {
        let (m, rest) = num(&command[15..])?;
        let position = rest.find("power")?;
        op(n.pow(m as u32), &rest[position + 5..])
    } else {
        None
    }
}

fn num(command: &str) -> Option<(i32, &str)> {
    let num_digits = command
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect::<String>();
    let n = num_digits.parse::<i32>().ok()?;
    Some((n, &command[num_digits.len()..]))
}
