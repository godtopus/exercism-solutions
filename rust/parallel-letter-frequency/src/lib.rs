extern crate chashmap;

use chashmap::CHashMap;
use std::thread;
use std::collections::HashMap;
use std::sync::Arc;

pub fn frequency(sequence: &'static [&'static str], n_threads: usize) -> HashMap<char, u32> {
    let counts: Arc<&CHashMap<char, u32>> = Arc::new(&CHashMap::new());
    let n_threads = match sequence.len() {
        0 => 1,
        len if len < n_threads => sequence.len(),
        _ => n_threads
    };

    let mut s = sequence.into_iter().map(|s| (*s).to_owned()).collect::<Vec<String>>();
    count(sequence, &mut counts.clone(), n_threads);

    counts.into_iter().collect()
}

fn count<'a>(sequence: &'static [&'static str], counts: &'a mut Arc<&'a CHashMap<char, u32>>, n_threads: usize) {
    let mut threads = Vec::with_capacity(n_threads);

    for window in sequence.chunks(n_threads) {
        //let counts = counts.clone();
        //let window = window.to_owned().clone();

        let f = |cnt : &'a Arc<&'a CHashMap<char, u32>>, w: &'a [&'a str]| {
            for s in window.iter() {
                for c in (*s).to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                    cnt.upsert(c, || 1, |e| *e += 1);
                }
            }
        };

        threads.push(thread::spawn(move || f(&counts, window)));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}