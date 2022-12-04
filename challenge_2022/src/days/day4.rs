use crate::days::day::{ChargementInput, Day};
use crate::string_tools::string_cast::CastTo;

pub struct Day4;

impl ChargementInput for Day4 {}

impl Day4 {

    fn transforme_ligne(chaine: String) -> (Range, Range) {
        chaine
            .split(",")
            .map(|chaine: &str| {
                let numbers = chaine.to_string()
                    .split("-")
                    .map(|chaine: &str| {
                        chaine.to_string().cast_to_or_panic()
                    }).collect::<Vec<u32>>();

                let range = numbers
                    .into_iter()
                    .fold((0, 0), |acc, number| {
                        if acc.0 == 0 {
                            (number, 0)
                        } else {
                            (acc.0, number)
                        }
                    });
                Range { min: range.0, max: range.1 }
            })
            .fold((Range { min: 0, max: 0 }, Range { min: 0, max: 0 }), |acc, range| {
                if acc.0.min == 0 {
                    (range, acc.1)
                } else {
                    (acc.0, range)
                }
            })
    }
}

impl Day for Day4 {
    fn day(&self) -> u8 { 4u8 }

    fn response_1(&self) -> String {
        self.clean_input().into_iter()
            .map(|chaine| Day4::transforme_ligne(chaine))
            .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
            .count()
            .to_string()
    }

    fn response_2(&self) -> String {
        self.clean_input().into_iter()
            .map(|chaine| Day4::transforme_ligne(chaine))
            .filter(|(r1, r2)| r1.overlaps(r2) || r2.overlaps(r1))
            .count()
            .to_string()
    }
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.min >= other.min && self.min <= other.max) || (self.max >= other.min && self.max <= other.max)
    }
}