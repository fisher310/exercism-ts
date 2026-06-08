pub fn get_diamond(c: char) -> Vec<String> {
    let order = c as i8 - b'A' as i8;
    (-order..=order)
        .map(|y| {
            (-order..=order)
                .map(|x| get_char(order, x, y))
                .collect::<String>()
        })
        .collect()
}

fn get_char(order: i8, x: i8, y: i8) -> char {
    if x.abs() + y.abs() == order {
        (x.unsigned_abs() + b'A') as char
    } else {
        ' '
    }
}
