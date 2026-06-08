pub fn is_armstrong_number(num: u32) -> bool {
      let x = num;
    let mut l = 0;
    let mut num = num;
    let mut list = vec![];
    while num != 0 {
        let n = num % 10;
        list.push(n);
        l += 1;
        num /= 10;
    }

    let mut sum = 0;
    for n in list {
        sum += n.pow(l);
    }

    x == sum
}
