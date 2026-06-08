pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let m = minefield.len() as i32;
    
    let n = minefield[0].chars().collect::<Vec<char>>().len() as i32;
    let x: Vec<i32> = vec![-1, 0, 1, 1, 1, 0, -1, -1];
    let y: Vec<i32> = vec![-1, -1, -1, 0, 1, 1, 1, 0];
    let mut ans = Vec::new();

    for i in 0..m {
        let mut line = String::new();
        for j in 0..n {
            if minefield[i as usize].chars().nth(j as usize).unwrap() == '*' {
                line.push('*');
                continue;
            }
            let mut num = 0;
            for k in 0..8 {
                let a = i as i32 + x[k];
                let b = j as i32 + y[k];
                if a >= 0
                    && a < m
                    && b >= 0
                    && b < n
                    && minefield[a as usize].chars().nth(b as usize).unwrap() == '*'
                {
                    num += 1;
                }
            }
            if num == 0 {
                line.push(' ');
            } else {
                line.push_str(&num.to_string());
            }
        }
        ans.push(line);
    }

    ans
}
