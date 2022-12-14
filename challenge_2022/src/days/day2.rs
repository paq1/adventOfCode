use crate::days::day::{ChargementInput, Day};

pub struct Day2;

impl ChargementInput for Day2 {}

impl Day2 {
    fn mapping_element(&self, lettre: char) -> Option<PlayEnum> {
        match lettre {
            'A' => Some(PlayEnum::Rock),
            'B' => Some(PlayEnum::Paper),
            'C' => Some(PlayEnum::Scissors),
            'X' => Some(PlayEnum::Rock),
            'Y' => Some(PlayEnum::Paper),
            'Z' => Some(PlayEnum::Scissors),
            _   => None
        }
    }

    fn win(mouv: Option<PlayEnum>) -> Option<PlayEnum> {
        match mouv {
            Some(PlayEnum::Rock) => Some(PlayEnum::Paper),
            Some(PlayEnum::Paper) => Some(PlayEnum::Scissors),
            Some(PlayEnum::Scissors) => Some(PlayEnum::Rock),
            _ => None
        }
    }

    fn draw(mouv: Option<PlayEnum>) -> Option<PlayEnum> {
        mouv
    }

    fn lose(mouv: Option<PlayEnum>) -> Option<PlayEnum> {
        match mouv {
            Some(PlayEnum::Rock) => Some(PlayEnum::Scissors),
            Some(PlayEnum::Paper) => Some(PlayEnum::Rock),
            Some(PlayEnum::Scissors) => Some(PlayEnum::Paper),
            _ => None
        }
    }

    fn kind_of_fn(mouv: Option<PlayEnum>) -> impl Fn(Option<PlayEnum>) -> Option<PlayEnum> {
        match mouv {
            Some(PlayEnum::Rock) => Day2::lose,
            Some(PlayEnum::Paper) => Day2::draw,
            Some(PlayEnum::Scissors) => Day2::win,
            _ => Day2::lose
        }
    }
}

impl Day for Day2 {
    fn day(&self) -> u8 { 2u8 }

    fn response_1(&self) -> String {

        let scores = self.clean_input()
            .iter()
            .map(|current_line: &String| {
                current_line
                    .split(" ")
                    .map(|chaine: &str| chaine.chars().nth(0).unwrap())
                    .map(|c| self.mapping_element(c))
                    .fold((None, None), |acc, current| {
                        if acc.0.is_none() {
                            (current, None)
                        } else {
                            (acc.0, current)
                        }
                    })
            })
            .into_iter()
            .map(|movement: (Option<PlayEnum>, Option<PlayEnum>)| {
                let mvt = (movement.0.unwrap(), movement.1.unwrap());
                let bonus_score = (mvt.0.get_value(), mvt.1.get_value());
                match mvt {
                    (a, b) if a == b => (3 + bonus_score.0, 3 + bonus_score.1),
                    (PlayEnum::Rock, PlayEnum::Scissors) => (6 + bonus_score.0, 0 + bonus_score.1),
                    (PlayEnum::Scissors, PlayEnum::Rock) => (0 + bonus_score.0, 6 + bonus_score.1),
                    (PlayEnum::Rock, PlayEnum::Paper) => (0 + bonus_score.0, 6 + bonus_score.1),
                    (PlayEnum::Paper, PlayEnum::Rock) => (6 + bonus_score.0, 0 + bonus_score.1),
                    (PlayEnum::Paper, PlayEnum::Scissors) => (0 + bonus_score.0, 6 + bonus_score.1),
                    (PlayEnum::Scissors, PlayEnum::Paper) => (6 + bonus_score.0, 0 + bonus_score.1),
                    _ => {
                        println!("pb mapping");
                        (0, 0)
                    }
                }
            })
            .fold((0, 0), |acc, current| (acc.0 + current.0, acc.1 + current.1));

        scores.1.to_string()
    }

    fn response_2(&self) -> String {
        let scores = self.clean_input()
            .iter()
            .map(|current_line: &String| {
                current_line
                    .split(" ")
                    .map(|chaine: &str| chaine.chars().nth(0).unwrap())
                    .map(|c| self.mapping_element(c))
                    .fold((None, None), |acc, current| {
                        if acc.0.is_none() {
                            (current, None)
                        } else {
                            (acc.0.clone(), Day2::kind_of_fn(current)(acc.0))
                        }
                    })
            })
            .into_iter()
            .map(|movement: (Option<PlayEnum>, Option<PlayEnum>)| {
                let mvt = (movement.0.unwrap(), movement.1.unwrap());
                let bonus_score = (mvt.0.get_value(), mvt.1.get_value());
                match mvt {
                    (a, b) if a == b => (3 + bonus_score.0, 3 + bonus_score.1),
                    (PlayEnum::Rock, PlayEnum::Scissors) => (6 + bonus_score.0, 0 + bonus_score.1),
                    (PlayEnum::Scissors, PlayEnum::Rock) => (0 + bonus_score.0, 6 + bonus_score.1),
                    (PlayEnum::Rock, PlayEnum::Paper) => (0 + bonus_score.0, 6 + bonus_score.1),
                    (PlayEnum::Paper, PlayEnum::Rock) => (6 + bonus_score.0, 0 + bonus_score.1),
                    (PlayEnum::Paper, PlayEnum::Scissors) => (0 + bonus_score.0, 6 + bonus_score.1),
                    (PlayEnum::Scissors, PlayEnum::Paper) => (6 + bonus_score.0, 0 + bonus_score.1),
                    _ => {
                        println!("pb mapping");
                        (0, 0)
                    }
                }
            })
            .fold((0, 0), |acc, current| (acc.0 + current.0, acc.1 + current.1));

        scores.1.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum PlayEnum {
    Rock,
    Paper,
    Scissors,
}

impl PlayEnum {
    fn get_value(&self) -> u32 {
        match self {
            PlayEnum::Rock     => 1,
            PlayEnum::Paper    => 2,
            PlayEnum::Scissors => 3
        }
    }
}
