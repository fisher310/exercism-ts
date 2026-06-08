#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|v| {
            let mut v = *v;
            if v == 0 {
                return vec![0];
            }
            let mut stream = Vec::new();
            let mut count = 0;
            while v != 0 {
                let mut tmp = v & 0b0111_1111;
                if count > 0 {
                    tmp |= 0b1000_0000;
                }
                stream.insert(0, tmp as u8);
                v >>= 7;
                count += 1;
            }
            stream
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = Vec::new();
    let mut tmp: u32 = 0;
    let mut finished = false;
    for b in bytes {
        let b = *b;
        if b & 0b1000_0000 == 0 {
            tmp = (tmp << 7) | b as u32;
            res.push(tmp);
            tmp = 0;
            finished = true;
        } else {
            tmp = (tmp << 7) | ((b << 1) >> 1) as u32;
            finished = false;
        }
    }

    if !finished {
        Err(Error::IncompleteNumber)
    } else {
        Ok(res)
    }
}

#[test]
fn test() {
    let x = "1, 0000000, 0000000";
    println!("{:0b}", 0x00004000);
}
