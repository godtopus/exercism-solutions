use std::collections::HashMap;

pub fn count(nucleotide: char, sequence: &str) -> Result<usize, &'static str> {
    let count_valid = sequence.chars().filter(|c| c == &nucleotide).count();
    let count_invalid = sequence.chars().filter(|&c| !is_nucleotide(c)).count();

    match count_invalid == 0 && is_nucleotide(nucleotide) {
        true => Ok(count_valid),
        false => Err("Sequence contains invalid nucleotides.")
    }
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, &'static str> {
    let mut counts = HashMap::<char, usize>::new();
    for nucleotide in "ACGT".chars() {
        let c = match count(nucleotide, sequence) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        counts.insert(nucleotide, c);
    }

    Ok(counts)
}

fn is_nucleotide(c: char) -> bool {
    c == 'A' || c == 'C' || c == 'G' || c == 'T'
}