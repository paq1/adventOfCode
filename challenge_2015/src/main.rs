use crate::days::day1::Day1;
use crate::days::day::{Day};

mod days;

fn main() {
    let d = Day1 {};

    println!("{:?}", d.response_1());
    println!("{:?}", d.response_2());
}
