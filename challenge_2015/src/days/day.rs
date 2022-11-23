use std::fs;

pub trait Day: ChargementInput1 + ChargementInput2 {
    fn day(&self) -> u8;

    fn input1(&self) -> Vec<String> {
        self.load_input_1(self.day())
    }

    fn input2(&self) -> Vec<String> {
        self.load_input_2(self.day())
    }
}

pub trait ChargementInput1: ChargementInput {
    fn load_input_1(&self, day: u8) -> Vec<String> {
        self.load_input(format!("inputs/day{day}/input1.txt").as_str())
    }
}

pub trait ChargementInput2: ChargementInput {
    fn load_input_2(&self, day: u8) -> Vec<String> {
        self.load_input(format!("inputs/day{day}/input2.txt").as_str())
    }
}

pub trait ChargementInput {
    fn load_input(&self, path: &str) -> Vec<String> {
        let chaines_opt = fs::read_to_string(path)
            .ok()
            .map(|chaine| chaine.split("\n").map(|c| c.to_string()).collect::<Vec<String>>());

        match chaines_opt {
            Some(chaines) => chaines,
            None => vec![]
        }
    }
}
