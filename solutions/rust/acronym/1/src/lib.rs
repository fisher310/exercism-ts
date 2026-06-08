pub fn abbreviate(phrase: &str) -> String {
    let mut ans = String::new();
    let phrase: String = phrase
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace() || *c == '-')
        .collect();

    let worlds = phrase.split(" ").map(|s| s.trim()).flat_map(|s| s.split('-')).collect::<Vec<&str>>();
    for word in worlds {
        if word.is_empty() {
            continue;
        }
        if word.chars().all(|c| c.is_uppercase()) {
            let c = word.chars().nth(0);
            match c {
                Some(v) => {
                    ans.push(v);
                }
                None => {}
            }
        } else {
            ans.push(word.chars().nth(0).unwrap().to_ascii_uppercase());
            let sw = word.chars().skip(1).collect::<Vec<char>>();
            if !sw.is_empty() {
                for c in sw {
                    if c.is_uppercase() {
                        ans.push(c);
                    }
                }
            }
        }
    }

    ans
}
