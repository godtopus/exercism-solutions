pub fn lsp(sequence: &str, n: usize) -> Result<u32, &'static str> {
    if sequence.len() < n || !sequence.chars().all(char::is_numeric) {
        return Err("Invalid sequence.");
    } else if n == 0 {
        return Ok(1);
    }

    Ok(sequence.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>().windows(n).map(|window| window.iter().product()).max().unwrap())
}