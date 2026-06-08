use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let letters = "ATGC";
    if !letters.contains(nucleotide) {
        return Err(nucleotide);
    }

    let res = dna.chars().find(|c|!letters.contains(*c));
    
    if let Some(c) = res {
        return Err(c);
    }

    let count = dna.chars().filter(|c| *c == nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let letters = "ATGC";

    let res = dna.chars().find(|c| !letters.contains(*c));
    if let Some(c) = res {
        return Err(c);
    }

    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('T', 0);
    map.insert('G', 0);
    map.insert('C', 0);

    dna.chars().for_each(|c| {
        (*map.entry(c).or_insert(0))  += 1;
    });

    
    Ok(map)
}
