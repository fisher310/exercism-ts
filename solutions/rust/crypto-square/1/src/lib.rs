pub fn encrypt(input: &str) -> String {
    let letters: Vec<char> = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    let c = (letters.len() as f64).sqrt().ceil() as usize;
    let r = (letters.len() as f64 / c as f64).ceil() as usize;

    let mut rectangle = vec![vec![' '; r]; c];

    for (i, &letter) in letters.iter().enumerate() {
        rectangle[i % c][i / c] = letter;
    }

    // println!("{:?}", rectangle);

    rectangle
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
