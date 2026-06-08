use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
let mut base = word.to_lowercase().chars().collect::<Vec<char>>();
    base.sort_unstable();


    possible_anagrams.iter()
        .filter(|&pa| {
            if word.len() != pa.len() || word.to_lowercase().eq(&pa.to_lowercase()) {
                return false;
            }
            let mut target = pa.to_lowercase().chars().collect::<Vec<char>>();
            target.sort_unstable();
            for i in 0..base.len() {
                if base[i] != target[i] {
                    return false;
                }
            }

            true
        }).copied().collect()
}
