use std::collections::HashMap;

pub struct Protein {
    pairs: HashMap<String, String>
}

impl Protein {
    pub fn name_for(&self, codon: &str) -> Result<&str, ()> {
        match self.pairs.get(codon) {
            Some(name) => Ok(name),
            _ => Err(())
        }
    }

    pub fn of_rna(&self, sequence: &str) -> Result<Vec<&str>, ()> {
        let mut translated = Vec::new();

        for codon in sequence.chars().collect::<Vec<char>>().chunks(3).map(|codon| codon.iter().collect::<String>()) {
            match self.pairs.get(&codon) {
                Some(name) => translated.push(name.as_str()),
                None => return Err(())
            }
        }

        Ok(translated.into_iter().take_while(|name| !name.eq(&"stop codon")).collect())
    }
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Protein {
    Protein {
        pairs: pairs.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()
    }
}