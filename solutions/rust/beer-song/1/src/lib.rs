pub fn verse(n: u32) -> String {
    match n {
        0 => {
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
        }

        1 => {
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
        }
        2 => {
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n - 1)
        }
        3..=99 => {
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n - 1)
        }
        _ => {
            panic!("invalid number")
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ans = String::new();
    let mut x: i32 = start as i32;
    while x >= end as i32 {
        ans.push_str(verse(x as u32).as_str());
        if x > end as i32 {
            ans.push('\n');
        }
        x -= 1
    }
    ans
}