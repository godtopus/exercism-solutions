extern crate regex;
use regex::Regex;

pub fn abbreviate(sequence: &str) -> String {
    let mut acronym = String::new();
    let re = Regex::new(r"\b\w|[A-Z][a-z]").unwrap();
    for word in re.captures_iter(sequence) {
        acronym.push_str(word[0][..1].into());
    }

    acronym.to_uppercase()
}