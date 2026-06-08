#[derive(Debug)]
pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut pt = PascalsTriangle { rows: vec![] };
        for i in 0..row_count {
            if i == 0 {
                pt.rows.push(vec![1]);
            } else if i == 1 {
                pt.rows.push(vec![1, 1]);
            } else {
                let pre = &pt.rows[(i - 1) as usize];
                let mut row = vec![1];
                for j in 1..pre.len() {
                    row.push(pre[j] + pre[j - 1]);
                }
                row.push(1);
                pt.rows.push(row);
            }
        }

        pt
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}