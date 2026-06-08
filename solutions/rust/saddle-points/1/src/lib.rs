use std::vec;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return vec![];
    }
    let mut row = vec![vec![0]; input.len()];
    let mut col = vec![vec![0]; input[0].len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            match input[i][row[i][0]].cmp(&input[i][j]) {
                std::cmp::Ordering::Less => {
                    row[i].clear();
                    row[i].push(j);
                }
                std::cmp::Ordering::Equal => {
                    if row[i][0] != j {
                        row[i].push(j);
                    }
                }
                _ => {}
            }
            match input[col[j][0]][j].cmp(&input[i][j]) {
                std::cmp::Ordering::Greater => {
                    col[j].clear();
                    col[j].push(i);
                }
                std::cmp::Ordering::Equal => {
                    if col[j][0] != i {
                        col[j].push(i);
                    }
                }
                std::cmp::Ordering::Less => {}
            }
        }
    }

    let mut res = Vec::new();
    for i in 0..row.len() {
        for c in &row[i] {
            if !col[*c].is_empty() && col[*c].contains(&i) {
                res.push((i, *c));
            }
        }
    }

    res
}
