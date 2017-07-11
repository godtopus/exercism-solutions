use std::ascii::AsciiExt;

pub fn encode(sequence: &str) -> String {
    sequence.to_lowercase()
            .chars()
            .filter(|ch| ch.is_alphanumeric() && ch.is_ascii())
            .map(|ch| convert(ch).to_string())
            .collect::<Vec<_>>()
            .chunks(5)
            .map(|chunk| chunk.join(""))
            .collect::<Vec<_>>()
            .join(" ")
}

pub fn decode(sequence: &str) -> String {
    sequence.replace(" ", "").chars().map(|ch| convert(ch)).collect::<String>()
}

fn convert(ch: char) -> char {
    let (a, z, c) = ('a' as u8, 'z' as u8, ch as u8);

    match ch {
        'a'...'z' => (z - c + a) as char,
        _ => ch,
    }
}