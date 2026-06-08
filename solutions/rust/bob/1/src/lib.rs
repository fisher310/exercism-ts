pub fn reply(message: &str) -> &str {
    let question = message.trim().ends_with("?");
    let chars = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();
    let captial = chars.len() > 0 && chars.iter().all(|c| c.is_uppercase());
    let blank = message.is_empty() || message.chars().all(|c| c.is_whitespace());
    if question && captial {
        "Calm down, I know what I'm doing!"
    } else if question {
        "Sure."
    } else if blank {
        "Fine. Be that way!"
    } else if captial {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
