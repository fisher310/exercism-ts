use crate::Allergen::{Cats, Chocolate, Eggs, Peanuts, Pollen, Shellfish, Strawberries, Tomatoes};

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Eggs => self.0 & Eggs as u32 > 0,
            Peanuts => self.0 & Peanuts as u32 > 0,
            Shellfish => self.0 & Shellfish as u32 > 0,
            Strawberries => self.0 & Strawberries as u32 > 0,
            Tomatoes => self.0 & Tomatoes as u32 > 0,
            Chocolate => self.0 & Chocolate as u32 > 0,
            Pollen => self.0 & Pollen as u32 > 0,
            Cats => self.0 & Cats as u32 > 0,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut res = Vec::new();
        if self.is_allergic_to(&Eggs) {
            res.push(Eggs);
        }
        if self.is_allergic_to(&Peanuts) {
            res.push(Peanuts);
        }
        if self.is_allergic_to(&Shellfish) {
            res.push(Shellfish);
        }
        if self.is_allergic_to(&Strawberries) {
            res.push(Strawberries);
        }
        if self.is_allergic_to(&Tomatoes) {
            res.push(Tomatoes);
        }
        if self.is_allergic_to(&Chocolate) {
            res.push(Chocolate);
        }
        if self.is_allergic_to(&Pollen) {
            res.push(Pollen);
        }
        if self.is_allergic_to(&Cats) {
            res.push(Cats);
        }

        res
    }
}
