use std::collections::HashMap;

pub fn word_count(sequence: &str) -> HashMap<String, u32> {
    let mut words = HashMap::<String, u32>::new();

    sequence.to_lowercase().split_whitespace().map(|word| {
        let w = word.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
        if !w.eq(&"") {
            *words.entry(w).or_insert(0) += 1;
        }
    }).count();

    words
}