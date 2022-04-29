#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // unimplemented!("Construct new Dna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", dna);
        let valid_str = String::from("ACGT");
        for (i, c) in dna.to_string().chars().enumerate() {
            if !valid_str.contains(c) {
                return Err(i as usize);
            }
        }
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        // unimplemented!("Transform Dna {:?} into corresponding Rna", self);
        let mut result = String::new();
        for c in self.0.chars() {
            result += match c {
                'G' => "C",
                'C' => "G",
                'A' => "U",
                'T' => "A",
                _ => continue,
            };
        }
        Rna(result)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // unimplemented!("Construct new Rna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
        let valid_str = String::from("GCAU");
        for (i, c) in rna.to_string().chars().enumerate() {
            if !valid_str.contains(c) {
                return Err(i);
            }
        }
        Ok(Rna(rna.to_string()))
    }
}
