extern crate itertools;
use itertools::Itertools;

extern crate permutohedron;
use permutohedron::*;

use std::collections::HashMap;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let mut equation: Vec<String> = puzzle.to_owned().replace(" ", "").split("==").map(|a| a.to_owned()).collect();
    let right_hand: String = equation.pop().unwrap();
    let left_hand: Vec<String> = equation.pop().unwrap().split("+").map(|a| a.to_owned()).collect();

    if left_hand.len() == 1 && left_hand.last().unwrap().ne(&right_hand) {
        return None
    } else if left_hand.iter().any(|a| a.len() > right_hand.len()) {
        return None
    }

    for solution_candidate in all_combinations(puzzle) {
        let right_hand_transformed = transform(&solution_candidate, &right_hand, right_hand.len());

        if right_hand_transformed == None || left_hand.iter().map(|s| transform(&solution_candidate, &s, s.len())).any(|n| n == None) {
            continue;
        }

        let left_hand_transformed: u32 = left_hand.iter().map(|s| transform(&solution_candidate, &s, s.len()).unwrap()).sum();

        match (left_hand_transformed, right_hand_transformed) {
            (l, Some(r)) if l == r => return Some(solution_candidate),
            _ => ()
        }
    }

    None
}

fn all_combinations(puzzle: &str) -> Vec<HashMap<char, u8>> {
    let mut unique: Vec<char> = puzzle.chars().filter(|c| c.is_alphabetic()).unique().collect();
    let no_unique = unique.len();
    let mut permutations = Vec::new();

    heap_recursive(&mut unique, |permutation| {
        permutations.push(permutation.to_vec())
    });

    let mut all_combinations: Vec<HashMap<char, u8>> = Vec::new();
    for ith_combination in (0..10).combinations(no_unique) {
        if ith_combination.iter().unique().count() != no_unique {
            continue;
        }

        for permutation in permutations.iter() {
            all_combinations.push(permutation.iter().zip(ith_combination.iter()).map(|(c, n)| (*c, *n)).collect::<HashMap<char, u8>>());
        }
    }

    all_combinations
}

fn transform(mapping: &HashMap<char, u8>, sequence: &str, len: usize) -> Option<u32> {
    let without_leading_zeroes = sequence.chars().map(|c| mapping.get(&c).unwrap()).skip_while(|c| *c == &0).count();
    if without_leading_zeroes != len {
        return None
    }

    Some(sequence.chars()
                .map(|c| mapping.get(&c).unwrap())
                .enumerate()
                .fold(0, |a, (i, n)| a + ((*n as u32) * 10_u32.pow((len - i - 1) as u32))))
}