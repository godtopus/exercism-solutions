use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::channel;

pub fn frequency(sequence: &[&str], n_threads: usize) -> HashMap<char, usize> {
    let seq: Vec<Vec<String>> = sequence.iter()
                                        .map(|s| (*s).to_string().to_lowercase())
                                        .collect::<Vec<String>>()
                                        .chunks(n_threads)
                                        .map(|s| s.iter().map(|s| s.to_string()).collect::<Vec<String>>())
                                        .collect();

    let mut handles = Vec::new();
    let (tx, rx) = channel();

    for chunk in seq {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            tx.send(count(&chunk)).unwrap();
        }));
    }

    let mut results: HashMap<char, usize> = HashMap::new();
    for _ in 0..handles.len() {
        for (c, n) in rx.recv().unwrap().into_iter() {
            *results.entry(c).or_insert(0) += n;
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    results
}

fn count(sequence: &Vec<String>) -> HashMap<char, usize> {
    let mut results: HashMap<char, usize> = HashMap::new();

    for c in sequence.concat().chars().filter(|c| c.is_alphabetic()) {
        *results.entry(c).or_insert(0) += 1;
    }

    results
}