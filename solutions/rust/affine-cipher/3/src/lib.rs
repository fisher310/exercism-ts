/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    println!("Encode {plaintext} with the key ({a}, {b})");

    if !is_coprime(a, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let encoded_text = plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| encode_char(c, a, b))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(encoded_text)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let a_inverse = mod_inverse(a, M);
    if a_inverse == -1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut decoded_text = String::new();
    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let decoded_char = decode_char(c, a_inverse, b);
            decoded_text.push(decoded_char);
        } else if c.is_ascii_digit() {
            decoded_text.push(c);
        }
    }

    Ok(decoded_text)
}

fn encode_char(c: char, a: i32, b: i32) -> char {
    match c {
        '0'..='9' => c,
        _ => {
            let encoded_char = (a * (c as i32 - 'a' as i32) + b).rem_euclid(M);
            char::from((encoded_char + 'a' as i32) as u8)
        }
    }
}

fn is_coprime(a: i32, b: i32) -> bool {
    gcd(a, b) == 1
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn mod_inverse(a: i32, m: i32) -> i32 {
    let mut s = 0;
    let mut old_s = 1;
    let mut r = m;
    let mut old_r = a;

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    if old_r != 1 {
        return -1;
    }
    
    (old_s % m + m) % m
}

fn decode_char(c: char, a_inverse: i32, b: i32) -> char {
    let x = (c as i32 - 'a' as i32 - b).rem_euclid(M);
    let decoded_char = (a_inverse * x).rem_euclid(M);
    char::from((decoded_char + 'a' as i32) as u8)
}

