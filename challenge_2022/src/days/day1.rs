use crate::days::day::{ChargementInput, Day};
use crate::string_tools::string_cast::CastTo;
use crate::vector_tools::sort::Sort;

pub struct Day1;

impl ChargementInput for Day1 {}

impl Day1 {

    fn get_elves_calories(&self) -> Vec<i32> {
        self.clean_input().iter()
            .fold(vec![], |acc, current| {
                if acc.is_empty() {
                    let current_int: i32 = current.cast_to_or_panic();
                    vec![current_int]
                } else {
                    if current.is_empty() {
                        [acc, vec![0]].concat()
                    } else {
                        let last_elem: i32 = acc.last()
                            .map(|v| {
                                let current_int: i32 = current.cast_to_or_panic();
                                v + current_int
                            })
                            .unwrap();

                        [acc[0..acc.len() - 1].to_vec(), vec![last_elem]].concat()
                    }
                }
            })
    }
}

impl Day for Day1 {
    fn day(&self) -> u8 { 1u8 }

    fn response_1(&self) -> String {
        let calories = self.get_elves_calories();
        let highest = calories.iter()
            .max();
        match highest {
            Some(a) => a.to_string(),
            None => "???".to_string()
        }
    }

    fn response_2(&self) -> String {
        // todo rendre calories immutable (adapter les methods sort et reverse)
        let calories: Vec<i32> = self.get_elves_calories()
            .sort_immut()
            .into_iter()
            .rev()
            .collect::<Vec<i32>>();
        //calories.reverse();
        let sum: i32 = calories[0..3].to_vec().iter().sum();
        sum.to_string()
    }
}