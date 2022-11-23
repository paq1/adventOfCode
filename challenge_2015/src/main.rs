use std::fs;

fn read_input(path: &str) -> Vec<String> {
    let chaines_opt = fs::read_to_string(path)
        .ok()
        .map(|chaine| chaine.split("\n").map(|c| c.to_string()).collect::<Vec<String>>());

    match chaines_opt {
        Some(chaines) => chaines,
        None => vec![]
    }
}

fn main() {
    let input = read_input("inputs/day1/input1.txt");
    input
        .into_iter()
        .enumerate()
        .for_each(|inp| println!("[index:{}] {}", inp.0, inp.1));
}
