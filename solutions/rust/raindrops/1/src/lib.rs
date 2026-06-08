pub fn raindrops(n: u32) -> String {
    let mut ans = String::new();
    if n % 3 == 0 {
        ans.push_str("Pling");
    }
    if n % 5 == 0 {
        ans.push_str("Plang");
    }
    if n % 7 == 0 {
        ans.push_str("Plong");
    }

    if ans.len() == 0 {
        n.to_string()
    } else {
        ans
    }}
