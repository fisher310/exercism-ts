pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let all_students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let idx = all_students.iter().position(|&s| s == student).unwrap() * 2;

    diagram
        .lines()
        .flat_map(|line| {
            line[idx..=idx + 1].chars().map(|p| match p {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                _ => "violets",
            })
        })
        .collect()
}
