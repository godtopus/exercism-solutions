/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();

    for value in values {
        let mut bytes: Vec<u8> = Vec::new();
        let mut current = *value;

        while current > 0 {
            if bytes.is_empty() {
                bytes.push((current & 0x7f) as u8);
            } else {
                bytes.push((0x80 | (current & 0x7f)) as u8);
            }

            current = current >> 7;
        }

        if bytes.is_empty() || bytes.first().unwrap() & 0x80_u8 != 0 {
            bytes.push(0);
        }


        encoded.extend(bytes.iter().rev());
    }

    encoded
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut decoded: Vec<u32> = Vec::new();
    let mut current: u32 = 0;

    for byte in bytes {
        if current > 0xfffffff - 0x7f {
            return Err("The value exceeds the maximum allowed.")
        }

        current = (current << 7_u32) | ((*byte as u32) & 0x7f_u32);

        if *byte & 0x80_u8 == 0_u8 {
            decoded.push(current);
            current = 0;
        }
    }

    if bytes.last().unwrap() & 0x80_u8 != 0 {
        return Err("Incomplete byte sequence.")
    }

    Ok(decoded)
}
