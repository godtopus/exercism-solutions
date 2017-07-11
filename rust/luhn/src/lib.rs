pub fn is_valid(sequence: &str) -> bool {
    let trimmed = sequence.replace(" ", "");
    let valid = trimmed.chars().all(char::is_numeric);
    if !valid || trimmed.len() <= 1 {
        return false
    }

    let sum = trimmed.chars().rev().enumerate().map(|(k, v)| {
        let mut value = v.to_digit(10).unwrap();

        if k % 2 != 0 {
            value = value * 2;
            if value > 9 {
                value -= 9;
            }
        }

        value
    }).sum::<u32>();

    sum % 10 == 0
}