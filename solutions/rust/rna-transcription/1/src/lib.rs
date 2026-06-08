#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut res = String::new();
        for (idx, nucleotidy) in dna.chars().enumerate() {
            match nucleotidy {
                'A' | 'T' | 'G' | 'C' => res.push(nucleotidy),
                _ => return Err(idx),
            }
        }

        Ok(Dna(res))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|ch| match ch {
                'A' => "U",
                'C' => "G",
                'T' => "A",
                'G' => "C",
                _ => "",
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut res = String::new();
        for (idx, ch) in rna.chars().enumerate() {
            match ch {
                'A' | 'C' | 'G' | 'U' => res.push(ch),
                _ => return Err(idx),
            }
        }
        Ok(Rna(res))
    }
}
