#[derive(Debug)]
pub struct Matrix {
    // Implement your Matrix struct
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let data: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
        Matrix { data }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no < 1 || row_no > self.data.len() {
            return None;
        }

        let row_no = row_no - 1;
        let row: Vec<u32> = self.data[row_no].iter().map(|n| *n).collect();
        Some(row)
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if self.data.len() < 1 {
            return None;
        }

        if col_no < 1 || col_no > self.data[0].len() {
            return None;
        }

        let col_no = col_no - 1;
        let col: Vec<u32> = self.data.iter().map(|row| row[col_no]).collect();
        Some(col)
    }
}
