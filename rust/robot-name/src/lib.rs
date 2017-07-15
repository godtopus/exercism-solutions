extern crate rand;

use rand::Rng;
use rand::OsRng;
use std::char;

pub struct Robot {
    name: String
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            name: Robot::generate_name()
        }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name();
    }

    fn generate_name() -> String {
        let mut rng = OsRng::new().unwrap();
        let mut name = String::new();

        name.push(rng.gen_range(b'A', b'Z' + 1) as char);
        name.push(rng.gen_range(b'A', b'Z' + 1) as char);
        name.push(char::from_digit(rng.gen_range(0, 10), 10).unwrap());
        name.push(char::from_digit(rng.gen_range(0, 10), 10).unwrap());
        name.push(char::from_digit(rng.gen_range(0, 10), 10).unwrap());

        name
    }
}