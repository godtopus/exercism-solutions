pub struct Luhn {
    sequence: String
}

impl <T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            sequence: input.to_string()
        }
    }
}

impl Luhn {
    pub fn is_valid(self) -> bool {
        let trimmed = self.sequence.replace(" ", "");
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
}