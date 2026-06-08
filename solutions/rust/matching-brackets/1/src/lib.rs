pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let open = vec!['(', '[', '{'];
    let close = vec![')', ']', '}'];
    for c in string.chars() {
        for (_, left) in open.iter().enumerate() {
            if c == *left {
                stack.push(c);
                continue;
            }
        }
        for (idx, right) in close.iter().enumerate() {
            if c == *right {
                match stack.pop() {
                    Some(v) => {
                        if v != open[idx] {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
    }

    stack.is_empty()
}
