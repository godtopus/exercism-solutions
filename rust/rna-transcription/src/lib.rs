#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    seq: String
}

impl RibonucleicAcid {
    pub fn new(sequence: &str) -> RibonucleicAcid {
        RibonucleicAcid {
            seq: sequence.to_owned()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    seq: String
}

impl DeoxyribonucleicAcid {
    pub fn to_rna(&self) -> Result<RibonucleicAcid, &str> {
        if !self.seq.chars().all(|c| is_nucleotide(c)) {
            return Err("Invalid DNA-sequence.")
        }

        Ok(RibonucleicAcid::new(&self.transcribe()))
    }

    pub fn new(sequence: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid {
            seq: sequence.to_owned()
        }
    }

    fn transcribe(&self) -> String {
        self.seq.chars().map(|c| convert(c)).collect()
    }
}

fn is_nucleotide(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false
    }
}

fn convert(c: char) -> char {
    match c {
        'C' => 'G',
        'G' => 'C',
        'A' => 'U',
        'T' => 'A',
        _ => c,
    }
}