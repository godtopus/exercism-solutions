use std::ascii::AsciiExt;

pub fn anagrams_for(word: &str, candidates: &[&str]) -> Vec<String> {
    let mut anagrams: Vec<String> = Vec::new();

    for candidate in candidates {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut c = candidate.to_lowercase().chars().collect::<Vec<char>>();
        c.sort();

        let mut w = word.to_lowercase().chars().collect::<Vec<char>>();
        w.sort();

        if c == w {
            anagrams.push((*candidate).to_owned());
        }
    }

    anagrams
}