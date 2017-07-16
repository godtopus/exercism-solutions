use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let mut equation: Vec<String> = puzzle.to_owned().replace(" ", "").split("==").map(|a| a.to_owned()).collect();
    let right_hand: String = equation.pop().unwrap();
    let left_hand: Vec<String> = equation.pop().unwrap().split("+").map(|a| a.to_owned()).collect();

    if left_hand.len() == 1 && left_hand.last().unwrap().ne(&right_hand) {
        return None
    } else if left_hand.iter().any(|a| a.len() > right_hand.len()) {
        return None
    }

    all_pairs(puzzle);

    Some(HashMap::new())
}

fn all_pairs(puzzle: &str) -> HashSet<HashMap<char, u8>> {
    let unique: HashSet<char> = puzzle.chars().filter(|c| c.is_alphabetic()).collect();
    let pairs = HashMap::new();

    HashSet::new()
}