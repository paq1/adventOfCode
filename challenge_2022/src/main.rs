use crate::days::{
    day::Day,
    day1::Day1
};

mod days;
mod vector_tools;
mod string_tools;

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
