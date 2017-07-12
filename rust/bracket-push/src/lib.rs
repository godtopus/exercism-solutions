pub struct Brackets {
    sequence: Vec<char>
}

impl Brackets {
    pub fn from(sequence: & str) -> Brackets {
        Brackets {
            sequence: sequence.chars().collect()
        }
    }

    pub fn are_balanced(self) -> bool {
        let mut open_stack: Vec<char> = Vec::new();

        for ch in self.sequence.into_iter() {
            match ch {
                '(' => open_stack.push(')'),
                '{' => open_stack.push('}'),
                '[' => open_stack.push(']'),
                ')' | '}' | ']' => {
                    if open_stack.pop() != Some(ch) {
                        return false;
                    }
                }
                _ => (),
            }
        }

        open_stack.is_empty()
    }
}