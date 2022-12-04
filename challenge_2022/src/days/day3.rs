use crate::days::day::{ChargementInput, Day};

pub struct Day3;

impl ChargementInput for Day3 {}

impl Day3 {
    fn from_string_to_items(chaine: &String) -> (String, String) {
        let taille = chaine.len();
        (
            chaine[..taille / 2].to_string(),
            chaine[taille / 2..].to_string()
        )
    }

    fn collide(items: (String, String)) -> char {
        items.0.chars()
            .fold('?', |acc, current| {
                if items.1.contains(current) {
                    current
                } else {
                    acc
                }
            })
    }

    fn char_to_int(c: char) -> i32 {
        if (c as i32) >= ('a' as i32) && (c as i32) <= ('z' as i32) {
            (c as i32) - ('a' as i32) + 1
        } else {
            (c as i32) - ('A' as i32) + 26 + 1
        }
    }
}

impl Day for Day3 {
    fn day(&self) -> u8 { 3u8 }

    fn response_1(&self) -> String {
        let res = self.clean_input().iter()
            .map(|line| Day3::from_string_to_items(line))
            .map(|items| Day3::collide(items))
            .map(|c| Day3::char_to_int(c))
            .sum::<i32>();

        res.to_string()
    }

    fn response_2(&self) -> String {
        "???".to_string()
    }
}