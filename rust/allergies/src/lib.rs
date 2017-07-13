#[derive(Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats
}

pub struct Allergies {
    score: u32
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: score
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        [Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate, Allergen::Pollen, Allergen::Cats]
            .to_vec()
            .into_iter()
            .filter(|allergy| self.is_allergic_to(&allergy))
            .collect()
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & Allergies::to_score(allergen) == Allergies::to_score(allergen)
    }

    fn to_score(allergen: &Allergen) -> u32 {
        match *allergen {
            Allergen::Eggs =>           1,
            Allergen::Peanuts =>        2,
            Allergen::Shellfish =>      4,
            Allergen::Strawberries =>   8,
            Allergen::Tomatoes =>       16,
            Allergen::Chocolate =>      32,
            Allergen::Pollen =>         64,
            Allergen::Cats =>           128
        }
    }
}