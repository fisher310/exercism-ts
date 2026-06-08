pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rows: Vec<Vec<char>> = vec![vec![]; self.0 as usize];
        let mut row = 0;
        let mut going_down = true;
        for c in text.chars() {
            rows[row].push(c);
            if row == 0 {
                going_down = true;
            } else if row == self.0 as usize - 1 {
                going_down = false;
            }
            if going_down {
                row += 1;
            } else {
                row -= 1;
            }
        }
        rows.concat().iter().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let len = cipher.len();
        let mut matrix = vec![vec![false; len]; self.0 as usize];
        let mut row = 0;
        let mut going_down = true;

        for col in 0..len {
            matrix[row][col] = true;
            if row == 0 {
                going_down = true;
            } else if row == self.0 as usize - 1 {
                going_down = false;
            }

            if going_down {
                row += 1;
            } else {
                row -= 1;
            }
        }
        let cipher_chars = cipher.chars().collect::<Vec<char>>();

        let mut result = vec![' '; len];
        let mut current = 0;
        for row in 0..self.0 as usize {
            for col in 0..len {
                if matrix[row][col] {
                    result[col] = cipher_chars[current];
                    current += 1;
                }
            }
        }
        result.iter().collect()
    }
}
