use std::fs;

pub trait Day: ChargementInput {
    fn day(&self) -> u8;

    fn response_1(&self) -> String;
    fn response_2(&self) -> String;

    fn input(&self) -> Vec<String> { self.load_input(self.day()) }
}

pub trait ChargementInput {
    fn load_input(&self, day: u8) -> Vec<String> {
        let chaines_opt = fs::read_to_string(format!("inputs/day{day}.txt").as_str())
            .ok()
            .map(|chaine| chaine.split("\n").map(|c| c.to_string()).collect::<Vec<String>>());

        match chaines_opt {
            Some(chaines) => chaines,
            None => vec![]
        }
    }
}
