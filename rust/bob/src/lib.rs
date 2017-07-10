pub fn reply(msg: &str) -> &'static str {
    let msg = msg.trim();

    if msg.is_empty() {
        "Fine. Be that way!"
    } else if msg.chars().filter(|c| c.is_alphabetic()).count() > 0 && msg.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        "Whoa, chill out!"
    } else if msg.chars().last() == Some('?') {
        "Sure."
    } else {
        "Whatever."
    }
}