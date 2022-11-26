use crate::days::day::{ChargementInput, ChargementInput1, Day};
use crate::vector_tools::distinct::*;
pub struct Day3;
impl ChargementInput1 for Day3 {}
impl ChargementInput for Day3 {}

impl Day3 {
    fn get_input(&self) -> String {
        match self.input1().get(0) {
            Some(chaine) => chaine.clone(),
            None => "".to_string()
        }
    }
}

impl Day for Day3 {
    fn day(&self) -> u8 { 3u8 }

    fn response_1(&self) -> String {
        let input = self.get_input();

        let coordonnes = input.chars()
            .fold(vec![(0,0)], |acc, caracter| {
                let current_position = acc.last().unwrap().clone();
                match caracter {
                    '^' => [acc, vec![(current_position.0, current_position.1 + 1)]].concat(),
                    'v' => [acc, vec![(current_position.0, current_position.1 - 1)]].concat(),
                    '>' => [acc, vec![(current_position.0 + 1, current_position.1)]].concat(),
                    '<' => [acc, vec![(current_position.0 - 1, current_position.1)]].concat(),
                    _ => acc
                }
            })
            .distinct();


        coordonnes.len().to_string()
    }

    fn response_2(&self) -> String {
        let input = self.get_input();

        let coordonnes = input.chars()
            .fold(vec![((0,0), (0,0))], |acc, caracter| {
                let tour = acc.len() % 2;
                if tour == 0 {
                    let current_position = acc.last().unwrap().1.clone();
                    let autre = acc.last().unwrap().0.clone();
                    match caracter {
                        '^' => [acc, vec![(autre, (current_position.0, current_position.1 + 1))]].concat(),
                        'v' => [acc, vec![(autre, (current_position.0, current_position.1 - 1))]].concat(),
                        '>' => [acc, vec![(autre, (current_position.0 + 1, current_position.1))]].concat(),
                        '<' => [acc, vec![(autre, (current_position.0 - 1, current_position.1))]].concat(),
                        _ => acc
                    }
                } else {
                    let current_position = acc.last().unwrap().0.clone();
                    let autre = acc.last().unwrap().1.clone();
                    match caracter {
                        '^' => [acc, vec![((current_position.0, current_position.1 + 1), autre)]].concat(),
                        'v' => [acc, vec![((current_position.0, current_position.1 - 1), autre)]].concat(),
                        '>' => [acc, vec![((current_position.0 + 1, current_position.1), autre)]].concat(),
                        '<' => [acc, vec![((current_position.0 - 1, current_position.1), autre)]].concat(),
                        _ => acc
                    }
                }
            })
            .into_iter()
            .fold(vec![], |acc, (a, b)| [acc, vec![a, b]].concat())
            .distinct();

        coordonnes.len().to_string()
    }
}