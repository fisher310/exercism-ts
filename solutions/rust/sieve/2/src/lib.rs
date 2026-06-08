pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut res: Vec<(u64, bool)> = (0..=upper_bound).map(|n| (n, false)).collect();

    for i in 2..res.len() {
        if !res[i].1 {
            let n = res[i].0;
            // let mut j = 2;
            // let mut x = n * j;

            // while x <= upper_bound {
            //     if !res[x as usize].1 {
            //         res[x as usize].1 = true;
            //     }
            //     j += 1;
            //     x = n * j;
            // }

            (2..=upper_bound / n).for_each(|m| {
                let y = m * n;
                if !res[y as usize].1 {
                    res[y as usize].1 = true;
                }
            })
        }
    }

    res.iter()
        .skip(2)
        .filter(|(_, marked)| !marked)
        .map(|(n, _)| *n)
        .collect()
}
