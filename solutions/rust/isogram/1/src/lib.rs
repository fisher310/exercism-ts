
use std::collections::HashMap;
pub fn check(candidate: &str) -> bool {
    let mut map = HashMap::<char, i32>::new();

    for c in candidate.to_lowercase().chars() {
        if c != '-' && c != ' ' && map.contains_key(&c) {
            return false;
        }
        map.insert(c, 1);
    }

    true
}
