pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![vec![0];0]
    }
    let size: usize = size as usize;
    let mut res = vec![vec![0; size]; size];

    let mut left_bound = 0;
    let mut right_bound = size - 1;
    let mut upper_bound = 0;
    let mut down_bound = size - 1;

    let mut n = 1;

    while left_bound <= right_bound && upper_bound <= down_bound {
        for i in left_bound..=right_bound {
            res[upper_bound][i] = n;
            n += 1;
        }
        upper_bound += 1;
        if left_bound == right_bound {
            break;
        }

        for j in upper_bound..=down_bound {
            res[j][right_bound] = n;
            n += 1;
        }

        right_bound -= 1;

        for i in (left_bound + 1..=right_bound).rev() {
            res[down_bound][i] = n;
            n += 1;
        }

        for j in (upper_bound..=down_bound).rev() {
            res[j][left_bound] = n;
            n += 1;
        }
        down_bound -= 1;
        left_bound += 1;
    }

    res
}
