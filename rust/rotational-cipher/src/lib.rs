pub fn rotate(sequence: &str, rot: u8) -> String {
    sequence.chars()
            .map(|ch| rotate_char(ch, rot))
            .collect::<String>()
}

fn rotate_char(ch: char, rot: u8) -> char {
    let (a, A, c) = ('a' as u8, 'A' as u8, ch as u8);

    match ch {
        'a'...'z' => (((c - a + rot) % 26) + a) as char,
        'A'...'Z' => (((c - A + rot) % 26) + A) as char,
        _ => ch,
    }
}