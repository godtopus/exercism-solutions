use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool
        where Self: Display
    {
        let trimmed = self.to_string().replace(" ", "");
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

impl Luhn for &'static str {}
impl Luhn for String {}
impl Luhn for u8 {}
impl Luhn for u16 {}
impl Luhn for u32 {}
impl Luhn for u64 {}
impl Luhn for usize {}