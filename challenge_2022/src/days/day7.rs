use crate::days::day::{ChargementInput, Day};

pub struct Day7;

impl ChargementInput for Day7 {}

impl Day7 {
}

impl Day for Day7 {
    fn day(&self) -> u8 { 7u8 }

    fn response_1(&self) -> String {
        "???".to_string()
    }

    fn response_2(&self) -> String {
        "???".to_string()
    }
}