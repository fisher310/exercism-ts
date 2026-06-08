use std::collections::VecDeque;

pub fn translate(input: &str) -> String {
    if input.starts_with("xr") || input.starts_with("yt") {
        let mut ans = input.to_string();
        ans.push_str("ay");
        return ans;
    }

    let values: Vec<&str> = input.split(' ').map(|s| s.trim()).collect();

    let mut ans = String::new();
    for one in values {
        let q: VecDeque<char> = one.chars().collect();
        ans.push_str(&do_translate(q));
        ans.push(' ');
    }

    ans.trim_end().to_string()
}

fn do_translate(mut chars: VecDeque<char>) -> String {
    let vowels: Vec<char> = vec!['a', 'o', 'e', 'i', 'u'];
    let mut pre: Option<char> = None;
    while let Some(ch) = chars.front() {
        if vowels.contains(ch) {
            // rule 3
            if *ch == 'u' && pre.is_some() && pre.unwrap() == 'q' {
                chars.push_back(*ch);
                chars.pop_front();
            }
            // rule 1
            break;
        } else {
            // rule 4
            if *ch == 'y' && pre.is_some() {
                break;
            }
            let curr = chars.pop_front().unwrap();
            pre = Some(curr);
            chars.push_back(curr);
        }
    }

    chars.push_back('a');
    chars.push_back('y');
    String::from_iter(chars.iter())
}