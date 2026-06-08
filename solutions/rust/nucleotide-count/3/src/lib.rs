use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'T' | 'G' | 'C' => {}
        _ => {
            return Err(nucleotide);
        }
    }

    let mut count = 0;
    for ch in dna.chars() {
        match ch {
            _ if ch == nucleotide => {
                count += 1;
            }
            'A' | 'T' | 'G' | 'C' => {
                continue;
            }
            _ => return Err(ch),
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('T', 0);
    map.insert('G', 0);
    map.insert('C', 0);

    for ch in dna.chars() {
        match ch {
            'A' | 'T' | 'G' | 'C' => {
                (*map.entry(ch).or_insert(0)) += 1;
            }
            _ => return Err(ch),
        }
    }

    Ok(map)
}
