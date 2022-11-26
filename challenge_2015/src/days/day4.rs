use crate::days::day::{ChargementInput, Day};

pub struct Day4;

impl ChargementInput for Day4 {}

impl Day4 {
    pub fn get_first_line_input(&self) -> String {
        self.input().first().unwrap().to_string()
    }
}

impl Day for Day4 {
    fn day(&self) -> u8 { 4u8 }

    fn response_1(&self) -> String {
        self.get_first_line_input()
    }

    fn response_2(&self) -> String {
        self.get_first_line_input()
    }
}