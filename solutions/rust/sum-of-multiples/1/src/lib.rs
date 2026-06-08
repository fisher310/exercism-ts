pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0_u32;
    for i in 1..limit {
        for j in 0..factors.len() {
            if factors[j] !=0 && i % factors[j] == 0 {
                sum += i;
                break;
            }
        }
    }
    sum
}
