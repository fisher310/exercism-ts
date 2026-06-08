#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna
            .chars()
            .position(|nucleotidy| !matches!(nucleotidy, 'A' | 'T' | 'G' | 'C'))
        {
            Some(p) => Err(p),
            None => Ok(Dna(dna.to_string())),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|ch| match ch {
                'A' => 'U',
                'C' => 'G',
                'T' => 'A',
                'G' => 'C',
                _ => panic!(),
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna
            .chars()
            .position(|nucleotidy| !matches!(nucleotidy, 'A' | 'U' | 'G' | 'C'))
        {
            Some(p) => Err(p),
            None => Ok(Rna(rna.to_string())),
        }
    }
}
