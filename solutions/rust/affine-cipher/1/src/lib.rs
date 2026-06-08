/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    println!("Encode {plaintext} with the key ({a}, {b})");

    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut encoded_text = String::new();
    let mut count = 0;
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let encoded_char = encode_char(c, a, b);
            encoded_text.push(encoded_char);
            count += 1;
        } else if c.is_ascii_digit() {
            encoded_text.push(c);
            count += 1;
        }
        if count > 0 && count %  5 ==0 && !encoded_text.ends_with(' ') {
            encoded_text.push(' ');
        }
    }

    Ok(encoded_text.trim_end().to_string())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {

    let a_inverse = mod_inverse(a, 26);
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
    let base = if c.is_ascii_uppercase() { 'A' as i32 } else { 'a' as i32 };
    let c_value = c as i32 - base;
    let encoded_char = (a * c_value + b) % 26;
    ((encoded_char + base) as u8 as char).to_ascii_lowercase()
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
    for i in 1..m {
        if (a * i) % m == 1 {
            return i;
        }
    }
    -1
}

fn decode_char(c: char, a_inverse: i32, b: i32) -> char {
    println!("Decode {c} with the key ({a_inverse}, {b})");
    let decoded_char = (a_inverse * (c as i32 - 'a' as i32 - b)) % 26;
    println!("decoded_char: {decoded_char}");
    let delta = if decoded_char < 0 { 26 } else { 0 };
    ((decoded_char + delta + 'a' as i32) as u8 as char).to_ascii_lowercase()
}
