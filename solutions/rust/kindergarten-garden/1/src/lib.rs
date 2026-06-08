pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let arrs = diagram.split_ascii_whitespace().collect::<Vec<&str>>();

    let all_students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let mut res = Vec::new();
    (0..12).for_each(|i| {
        if all_students[i] == student {
            let temp = [
                arrs[0].chars().nth(i * 2).unwrap(),
                arrs[0].chars().nth(i * 2 + 1).unwrap(),
                arrs[1].chars().nth(i * 2).unwrap(),
                arrs[1].chars().nth(i * 2 + 1).unwrap(),
            ];
            for c in temp {
                match c {
                    'G' => {
                        res.push("grass");
                    }
                    'C' => {
                        res.push("clover");
                    }
                    'R' => {
                        res.push("radishes");
                    }
                    'V' => {
                        res.push("violets");
                    }
                    _ => {}
                }
            }
        }
    });

    res
}
