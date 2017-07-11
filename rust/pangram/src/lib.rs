use std::collections::HashSet;
use std::ascii::AsciiExt;

pub fn is_pangram(sentence: &'static str) -> bool {
    sentence.to_lowercase().chars().filter(|c| c.is_alphabetic() && c.is_ascii()).collect::<HashSet<char>>().len() == 26
}