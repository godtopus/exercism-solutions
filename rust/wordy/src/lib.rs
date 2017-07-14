extern crate regex;

use regex::Regex;

pub struct WordProblem {
    word: String
}

impl WordProblem {
    pub fn new(word: &str) -> Self {
        WordProblem {
            word: word.to_owned()
        }
    }

    pub fn answer(&self) -> Result<i64, ()> {
        let re = Regex::new(r"(-?\d+)|(plus|minus|multiplied|divided)|(-?\d+)").unwrap();
        let mut sum: i64 = 0;
        let mut op: String = "".to_owned();

        if re.captures_iter(&self.word).count() < 3 {
            return Err(())
        }

        for group in re.captures_iter(&self.word) {
            println!("Group: {}\n", &group[0]);
            match &group[0] {
                "plus" | "minus" | "multiplied" | "divided" => op = group[0].to_owned(),
                n => match op.as_ref() {
                    "plus" => sum = sum + n.parse::<i64>().unwrap(),
                    "minus" => sum = sum - n.parse::<i64>().unwrap(),
                    "multiplied" => sum = sum * n.parse::<i64>().unwrap(),
                    "divided" => sum = sum / n.parse::<i64>().unwrap(),
                    _ => sum = sum + n.parse::<i64>().unwrap()
                }
            }
        }

        Ok(sum)
    }
}