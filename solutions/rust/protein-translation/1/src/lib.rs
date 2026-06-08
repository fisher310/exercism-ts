use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    codon_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut chars = rna.chars().peekable();
        let mut res = Vec::new();
        while chars.peek().is_some() {
            let mut tmp = String::with_capacity(3);
            for _ in 0..3 {
                if let Some(ch) = chars.next() {
                    tmp.push(ch);
                } else {
                    return None;
                }
            }

            let name = self.codon_map.get(tmp.as_str()).copied();
            match name {
                Some("stop codon") => return Some(res),
                Some(n) => res.push(n),
                None => return None,
            }
        }

        Some(res)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut map = HashMap::new();
    for (codon, name) in pairs {
        map.insert(codon, name);
    }

    CodonsInfo { codon_map: map }
}
