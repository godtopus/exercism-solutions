extern crate permutohedron;

use permutohedron::heap_recursive;
use permutohedron::control::Control;

type Domino = (usize, usize);

pub fn chain(dominoes: &Vec<Domino>) -> Option<Vec<Domino>> {
    match dominoes.len() {
        0 => Some(vec!()),
        1 => {
            match dominoes[0] {
                (l, r) if l == r => Some(vec![dominoes[0]]),
                _ => None
            }
        }
        _ => {
            if dominoes.iter().fold([0, 0, 0, 0, 0, 0], |mut acc, &(l, r)| {
                acc[l - 1] += 1;
                acc[r - 1] += 1;
                acc
            }).iter().any(|n| n % 2 != 0) {
                return None
            }

            heap_recursive(&mut dominoes.clone().as_mut_slice(), |permutation| {
                match check_chain(permutation) {
                    Some(v) => Control::Break(v),
                    None => Control::Continue
                }
            }).break_value()
        }
    }
}

fn check_chain(permutation: &[Domino]) -> Option<Vec<Domino>> {
    let mut chain = Vec::new();

    match (permutation[0], permutation[permutation.len() - 1]) {
        ((l, _), (r1, r2)) if l == r1 || l == r2 => chain.push(permutation[0]),
        ((_, l), (r1, r2)) if l == r1 || l == r2 => chain.push((permutation[0].1, permutation[0].0)),
        _ => return None
    }

    let mut reversed = false;
    for window in permutation.to_vec().windows(2) {
        match (chain.last().unwrap().1, window[1].0, window[1].1) {
            (l, r1, _) if l == r1 => {
                reversed = false;
                chain.push(window[1]);
            },
            (l, _, r2) if l == r2 && !reversed => {
                reversed = true;
                chain.push((window[1].1, window[1].0));
            },
            _ => break
        }
    }

    if chain.len() == permutation.len() && chain[0].0 == chain[chain.len() - 1].1 {
        return Some(chain)
    }

    None
}