use crate::days::day::{ChargementInput, Day};

pub struct Day6;

impl ChargementInput for Day6 {}

impl Day6 {
    fn all_char_differents(chaine: String) -> bool {
        let res = chaine.chars()
            .enumerate()
            .map(|(index, c)| {
                //println!("index = {} et taille = {}", index, chaine.len());
                if index == chaine.len()-1 {
                    0
                } else if chaine[(index+1)..].to_string().contains(c) {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>();
        res == 0
    }
}

impl Day for Day6 {
    fn day(&self) -> u8 { 6u8 }

    fn response_1(&self) -> String {
        self.clean_input()
            .first()
            .map(|input| {
                input.chars()
                    .enumerate()
                    .fold((-1, true), |acc, (index, _)| {
                        if acc.1 {
                            if Day6::all_char_differents(input[..4].to_string()) {
                                (5, false)
                            } else {
                                (acc.0, false)
                            }
                        } else {
                            if index >= input.len() - 4 {
                                acc
                            } else if acc.0 == -1 {
                                let chaine_testee = input[index..index+4].to_string();

                                if Day6::all_char_differents(chaine_testee.clone()) {
                                    (4 + (index as i32), false)
                                } else {
                                    acc
                                }
                            } else {
                                acc
                            }
                        }
                    })
                    .0
            })
            .unwrap()
            .to_string()
    }

    fn response_2(&self) -> String {
        self.clean_input()
            .first()
            .map(|input| {
                input.chars()
                    .enumerate()
                    .fold((-1, true), |acc, (index, _)| {
                        if acc.1 {
                            if Day6::all_char_differents(input[..14].to_string()) {
                                (15, false)
                            } else {
                                (acc.0, false)
                            }
                        } else {
                            if index >= input.len() - 14 {
                                acc
                            } else if acc.0 == -1 {
                                let chaine_testee = input[index..index+14].to_string();

                                if Day6::all_char_differents(chaine_testee.clone()) {
                                    (14 + (index as i32), false)
                                } else {
                                    acc
                                }
                            } else {
                                acc
                            }
                        }
                    })
                    .0
            })
            .unwrap()
            .to_string()
    }
}