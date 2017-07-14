use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    word_defs: HashMap<String, String>,
    stack: Vec<Value>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            word_defs: [("dup", "dup"), ("drop", "drop"), ("swap", "swap") , ("over", "over")].iter().map(|&(a, b)| (a.into(), b.into())).collect(),
            stack: Vec::new()
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut found_open = false;
        let mut found_word = Vec::new();

        println!("defs: {:?}", self.word_defs);

        for op in input.to_lowercase().split(|c: char| !c.is_alphanumeric() && c != '+' && c != '-' && c != '*' && c != '/' && c != ':' && c != ';') {
            match op {
                ":" if !found_open => found_open = true,
                ";" if found_open => found_open = false,
                word if found_open => {
                    if found_word.is_empty() {
                        found_word.push(word.to_owned());
                    } else {
                        self.word_defs.insert(found_word.clone().last().unwrap().to_owned(), word.into());
                    }
                },
                "+" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(b), Some(a)) => self.stack.push(a + b),
                        (_, _) => return Err(Error::StackUnderflow)
                    }
                },
                "-" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(b), Some(a)) => self.stack.push(a - b),
                        (_, _) => return Err(Error::StackUnderflow)
                    }
                },
                "*" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(b), Some(a)) => self.stack.push(a * b),
                        (_, _) => return Err(Error::StackUnderflow)
                    }
                },
                "/" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(0), Some(_)) => return Err(Error::DivisionByZero),
                        (Some(b), Some(a)) => self.stack.push(a / b),
                        (_, _) => return Err(Error::StackUnderflow)
                    }
                },
                "dup" => {
                    match self.stack.pop() {
                        Some(a) => {
                            self.stack.push(a);
                            self.stack.push(a);
                        },
                        None => return Err(Error::StackUnderflow)
                    }
                },
                "drop" => {
                    match self.stack.pop() {
                        Some(a) => (),
                        None => return Err(Error::StackUnderflow)
                    }
                },
                "swap" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(b), Some(a)) => {
                            self.stack.push(b);
                            self.stack.push(a);
                        },
                        (_, _) => return Err(Error::StackUnderflow)
                    }
                },
                "over" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(b), Some(a)) => {
                            self.stack.push(a);
                            self.stack.push(b);
                            self.stack.push(a);
                        },
                        (_, _) => return Err(Error::StackUnderflow)
                    }
                },
                n if n.parse::<i32>().is_ok() => self.stack.push(n.parse::<i32>().unwrap()),
                _ if !self.word_defs.contains_key(op) => return Err(Error::UnknownWord),
                _ => return Err(Error::InvalidWord)
            }
        }

        if found_open {
            return Err(Error::InvalidWord)
        }

        Ok(())
    }
}
