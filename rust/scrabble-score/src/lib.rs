pub fn score(word: &'static str) -> u32 {
    let mut score = 0;

    for c in word.to_lowercase().chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => score += 1,
            'd' | 'g' => score += 2,
            'b' | 'c' | 'm' | 'p' => score += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => score += 4,
            'k' => score += 5,
            'j' | 'x' => score += 8,
            'q' | 'z' => score += 10,
            _ => score += 0,
        }
    }

    score
}