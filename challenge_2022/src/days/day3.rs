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

    fn pack_3(items: &Vec<String>) -> Vec<(String, String, String)> {
        items.iter()
            .fold(vec![], |acc, current| {
                if acc.is_empty() {
                    vec![(Some(current.to_string()), None, None)]
                } else {
                    if acc.last().unwrap().1.is_none() {
                        [
                            acc[..acc.len() - 1].to_vec(),
                            vec![(acc.last().unwrap().0.clone(), Some(current.to_string()), None)]
                        ].concat()
                    } else if acc.last().unwrap().2.is_none() {
                        [
                            acc[..acc.len() - 1].to_vec(),
                            vec![
                                (
                                    acc.last().unwrap().0.clone(),
                                    acc.last().unwrap().1.clone(),
                                    Some(current.to_string())
                                )
                            ]
                        ].concat()
                    } else {
                        [acc, vec![(Some(current.to_string()), None, None)]].concat()
                    }
                }
            })
            .into_iter()
            .map(|item| (item.0.unwrap(), item.1.unwrap(), item.2.unwrap()))
            .collect::<Vec<(String, String, String)>>()
    }
    fn collide_3(items: (String, String, String)) -> char {
        items.0.chars()
            .fold('?', |acc, current| {
                if items.1.contains(current) && items.2.contains(current) {
                    current
                } else {
                    acc
                }
            })
    }
}

impl Day for Day3 {
    fn day(&self) -> u8 { 3u8 }

    fn response_1(&self) -> String {
        self.clean_input().iter()
            .map(|line| Day3::from_string_to_items(line))
            .map(|items| Day3::collide(items))
            .map(|c| Day3::char_to_int(c))
            .sum::<i32>()
            .to_string()
    }

    fn response_2(&self) -> String {
        Day3::pack_3(&self.clean_input()).into_iter()
            .map(Day3::collide_3)
            .map(Day3::char_to_int)
            .sum::<i32>()
            .to_string()
    }
}