use crate::days::day1::Day1;
use crate::days::day::{Day};

mod days;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day1 {})
    ];

    days
        .iter()
        .enumerate()
        .for_each(|(index, d)| {
            println!("day{} {:?}", index+1, d.response_1());
            println!("day{} {:?}", index+1, d.response_2());
        });
}
