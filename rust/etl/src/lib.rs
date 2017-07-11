use std::collections::BTreeMap;
use std::ascii::AsciiExt;

pub fn transform(scores: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_scores = BTreeMap::new();
    for key in scores.keys() {
        for value in scores.get(key).unwrap() {
            new_scores.insert(value.to_ascii_lowercase(), *key);
        }
    }

    new_scores
}