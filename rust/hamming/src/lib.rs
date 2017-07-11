pub fn hamming_distance(seq1: &str, seq2: &str) -> Result<u32, &'static str> {
    match seq1.len() == seq2.len() {
        true => {
            Ok(seq1.chars().zip(seq2.chars()).fold(0, |a, b|  {
                match b.0 == b.1 {
                    true => a,
                    false => a + 1
                }
            }))
        },
        false => Err("Sequences differ in length.")
    }
}