use crate::days::day::{ChargementInput, Day};

pub struct Day2;

impl ChargementInput for Day2 {}

impl Day2 {
    fn get_from_string_to_dimension(&self, chaine: &String, index_line: usize) -> Vec<i32> {
        chaine
            .split("x")
            .enumerate()
            .map(|(index, element)| {
                let transformed_element = element.to_string();

                let retrans = if index == 2 && index_line != self.input().len()-1 {
                    transformed_element[0..transformed_element.len()-1].to_string()
                } else {
                    transformed_element
                };

                match retrans.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => {
                        panic!("conversion impossible de {}", element);
                    }
                }
            })
            .collect::<Vec<i32>>()
    }
}

// todo reparer l'input

impl Day for Day2 {
    fn day(&self) -> u8 { 2u8 }

    fn response_1(&self) -> String {
        let res = self
            .input()
            .iter()
            .enumerate()
            .map(|(ligne, chaine)| {
                let dimensions = self.get_from_string_to_dimension(chaine, ligne);
                let (l, w, h) = if let [l, w, h] = dimensions[..] { (l, w, h) } else { (-1, -1, -1) };
                let mut sort = dimensions.clone();
                sort.sort();
                2*l*w + 2*w*h + 2*l*h + sort[0]*sort[1]
            })
            .sum::<i32>();

        format!("{}", res)
    }

    fn response_2(&self) -> String {
        let res = self
            .input()
            .iter()
            .enumerate()
            .map(|(ligne, chaine)| {
                let dimensions = self.get_from_string_to_dimension(chaine, ligne);
                let (l, w, h) = if let [l, w, h] = dimensions[..] { (l, w, h) } else { (-1, -1, -1) };
                let mut sort = dimensions.clone();
                sort.sort();
                2 * sort[0] + 2 * sort[1] + l * w * h
            })
            .sum::<i32>();

        format!("{}", res)
    }
}