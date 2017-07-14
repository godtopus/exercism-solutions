pub fn number(pn: &str) -> Option<String> {
    let mut clean: String = pn.chars().filter(|c| c.is_numeric()).collect();

    if clean.len() == 10 {
        return Some(clean)
    } else if clean.len() == 11 {
        return match clean.remove(0) {
            '1' => Some(clean),
            _ => None
        }
    }

    None
}

pub fn area_code(pn: &str) -> Option<String> {
    match number(pn) {
        Some(clean) => Some(clean[0..3].into()),
        _ => None
    }
}

pub fn pretty_print(pn: &str) -> String {
    match number(pn) {
        Some(clean) => format!("({}) {}-{}", &clean[0..3], &clean[3..6], &clean[6..]),
        _ => "invalid".into()
    }
}