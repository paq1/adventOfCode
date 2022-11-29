use crate::days::day::{ChargementInput, Day};
// use crate::vector_tools::distinct::Distinct;
pub struct Day5;

impl ChargementInput for Day5 {}

impl Day5 {

    fn voyelle_rule(chaine: &String) -> bool {
        let voyelles = "aeiou".to_string();
        chaine
            .chars()
            .filter(|c| voyelles.contains(*c))
            .collect::<Vec<char>>()
            //.distinct()
            .len() >= 3
    }

    fn twice_appears_rule(chaine: &String) -> bool {
        chaine
            .chars()
            .fold((false, ' '), |acc, current| {
                if current == acc.1 {
                    (true, current)
                } else {
                    (acc.0, current)
                }
            })
            .0
    }

    fn black_list_rule(chaine: &String) -> bool {
        let interdit = vec!["ab", "cd", "pq", "xy"];
        interdit
            .iter()
            .map(|c| chaine.contains(*c))
            .filter(|cond| *cond)
            .collect::<Vec<bool>>()
            .len() == 0
    }

    fn rules_part1(chaine: &String) -> bool {
        Self::voyelle_rule(chaine) &&
        Self::twice_appears_rule(chaine) &&
        Self::black_list_rule(chaine)
    }
}

impl Day for Day5 {
    fn day(&self) -> u8 { 5u8 }

    fn response_1(&self) -> String {
        let nb_valid = self.input()
            .iter()
            .map(Self::rules_part1)
            .filter(|cond| *cond)
            .collect::<Vec<bool>>()
            .len();

        format!("valid : {}", nb_valid)
    }

    fn response_2(&self) -> String {
        "???".to_string()
    }
}