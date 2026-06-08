pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let row_count = input.len();

    for (row_num, row) in input.iter().enumerate() {
        for (col_num, val) in row.iter().enumerate() {
            if row
                .iter()
                .all(|x| x <= val && (0..row_count).all(|x| input[x][col_num] >= *val))
            {
                res.push((row_num, col_num));
            }
        }
    }

    res
}
