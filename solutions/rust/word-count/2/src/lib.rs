use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut res = HashMap::new();

    words.split(&[' ', ',', '.', ':', '!', '?', '\t', '\r', '\n'])
        .map(|s| s.trim_matches(&['\'', '\"', '&', '@', '$', '%', '^']))
        .filter(|s| {
            !s.is_empty()
        })
        .map(|s| {
            String::from(s)
        })
        .map(|s| s.to_lowercase())
        .for_each(|s| {
            *res.entry(s).or_insert(0) += 1;
        });
    res
}
