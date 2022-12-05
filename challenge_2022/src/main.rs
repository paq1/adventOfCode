extern crate core;

use crate::days::{
    day::Day,
    day1::Day1,
    day2::Day2,
    day3::Day3,
    day4::Day4,
    day5::Day5,
    day6::Day6,
    day7::Day7,
    day8::Day8
};

mod days;
mod vector_tools;
mod string_tools;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day1 {}),
        Box::new(Day2 {}),
        Box::new(Day3 {}),
        Box::new(Day4 {}),
        Box::new(Day5 {}),
        Box::new(Day6 {}),
        Box::new(Day7 {}),
        Box::new(Day8 {})
    ];

    days
        .iter()
        .enumerate()
        .for_each(|(index, d)| {
            println!("day{} part 1 = {:?}", index+1, d.response_1());
            println!("day{} part 2 = {:?}", index+1, d.response_2());
        });
}
