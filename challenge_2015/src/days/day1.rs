use crate::days::day::{ChargementInput, ChargementInput1, Day};

pub struct Day1;

impl ChargementInput1 for Day1 {}
impl ChargementInput for Day1 {}

impl Day1 {
    pub fn transform_caracter(caracter: char) -> i32 {
        if caracter == '(' {1}
        else if caracter == ')' {-1}
        else {0}
    }
}

impl Day for Day1 {
    fn day(&self) -> u8 { 1u8 }

    fn response_1(&self) -> String {
        match self.input1().first() {
            Some(chaine) =>
                chaine.chars()
                    .map(Day1::transform_caracter )
                    .sum::<i32>()
                    .to_string()
            ,
            None => "xxx".to_string()
        }
    }

    fn response_2(&self) -> String {
        match self.input1().first() {
            Some(chaine) =>
                chaine.chars()
                    .map(Day1::transform_caracter )
                    .enumerate()
                    .fold((0,-1), |acc, (index, current)| {
                        let sum: i32 = acc.0 + current;
                        if acc.0 == -1 && acc.1 == -1 {
                            (sum, index as i32)
                        } else {
                            (sum, acc.1)
                        }
                    })
                    .1
                    .to_string()
            ,
            None => "xxx".to_string()
        }
    }
}