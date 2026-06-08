pub fn build_proverb(list: &[&str]) -> String {

    let mut ans = vec![];
    if list.is_empty() {
        return "".to_string();
    }

    for i in 0..list.len() - 1 {
        ans.push(format!("For want of a {} the {} was lost.", list[i], list[i + 1]))
    }
    ans.push(format!("And all for the want of a {}.", list[0]));
    ans.join("\n")
}