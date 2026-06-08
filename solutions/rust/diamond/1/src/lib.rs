pub fn get_diamond(c: char) -> Vec<String> {

    let index = c as u8 - b'A';
    let len = index * 2 + 1;

    let mut diamond = Vec::with_capacity(len as usize);

    let mut left = len / 2;
    let mut right = left;
    for i in 0..=index {
        let mut row = String::with_capacity(len as usize);

        for j in 0..len {
            if j == left || j == right {
                row.push((i + b'A') as char);
            } else {
                row.push(' ');
            }
        }
        diamond.push(row);
        left = left.saturating_sub(1);
        right = right.saturating_add(1);
    }

    for i in (0..index).rev() {
        diamond.push(diamond[i as usize].clone());
    }

    diamond
}
