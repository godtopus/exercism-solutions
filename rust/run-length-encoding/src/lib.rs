pub fn encode(data: &str) -> String {
    if data.is_empty() {
        return data.into()
    }

    let mut encoded = String::new();
    let mut previous = data.chars().nth(0).unwrap();
    let mut count = 1;

    for c in data.chars().skip(1) {
        if previous == c {
            count += 1;
        } else {
            encoded.push_str(&convert(count));
            encoded.push(previous);
            previous = c;
            count = 1;
        }
    }

    encoded.push_str(&convert(count));
    encoded.push(previous);

    encoded
}

pub fn decode(data: &str) -> String {
    let mut times: String = String::new();
    let mut decoded: String = String::new();

    for c in data.chars() {
        if c.is_numeric() {
            times.push(c);
        } else {
            decoded.push_str(&c.to_string().repeat(times.parse().unwrap_or(1)));
            times.clear();
        }
    }

    decoded
}

fn convert(n: u32) -> String {
    match n {
        1 => "".into(),
        _ => n.to_string()
    }
}