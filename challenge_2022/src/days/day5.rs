use crate::days::day::{ChargementInput, Day};
use crate::string_tools::string_cast::CastTo;

pub struct Day5;

impl ChargementInput for Day5 {}

impl Day5 {
    // fn mapping_instruction(&self) -> Vec<>
}

impl Day for Day5 {
    fn day(&self) -> u8 { 5u8 }

    fn response_1(&self) -> String {

        let instructions = self.clean_input().into_iter()
            .map(|line| Instruction::new(line))
            .collect::<Vec<_>>();

        let nouvelles_piles = instructions.into_iter()
            .fold(Pile::load_piles(), |acc, instruction| {
                instruction.modif(&acc)
            });

        let lettres = nouvelles_piles.into_iter()
            .map(|pile| pile.out().unwrap_or('?'))
            .collect::<Vec<_>>();

        let phrase = lettres.into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");

        phrase
    }

    fn response_2(&self) -> String {
        "???".to_string()
    }
}

#[derive(Debug, Clone)]
struct Pile {
    id: u8,
    elements: Vec<char>
}
impl Pile {
    fn load_piles_test() -> Vec<Pile> {
        vec![
            Pile {
                id: 1,
                elements: vec!['Z', 'N']
            },
            Pile {
                id: 2,
                elements: vec!['M', 'C', 'D']
            },
            Pile {
                id: 3,
                elements: vec!['P']
            },
        ]
    }

    fn load_piles() -> Vec<Pile> {
        vec![
            Pile {
                id: 1,
                elements: vec!['B', 'Q', 'C']
            },
            Pile {
                id: 2,
                elements: vec!['R', 'Q', 'W', 'Z']
            },
            Pile {
                id: 3,
                elements: vec!['B', 'M', 'R', 'L', 'V']
            },
            Pile {
                id: 4,
                elements: vec!['C', 'Z', 'H', 'V', 'T', 'W']
            },
            Pile {
                id: 5,
                elements: vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G']
            },
            Pile {
                id: 6,
                elements: vec!['H', 'N', 'P', 'C', 'J', 'F', 'V','Q']
            },
            Pile {
                id: 7,
                elements: vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S']
            },
            Pile {
                id: 8,
                elements: vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P']
            },
            Pile {
                id: 9,
                elements: vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P']
            },
        ]
    }

    fn out(&self) -> Option<char> {
        self.elements.last().map(|v| v.clone())
    }

    fn drop_last(&self) -> Pile {
        Pile {
            id: self.id,
            elements: self.elements[..self.elements.len() - 1].to_vec()
        }
    }

    fn add(&self, carac: char) -> Pile {
        Pile {
            id: self.id,
            elements: [self.elements.clone(), vec![carac]].concat()
        }
    }
}

#[derive(Debug)]
struct Instruction {
    r#move: u8,
    from: u8,
    to: u8
}
impl Instruction {
    fn new(chaine: String) -> Instruction {
        let numbers = chaine.split(" ")
            .fold(vec![], |acc, current| {
                let number_opt: Option<u8> = current.to_string().cast_to_opt();
                if number_opt.is_none() {
                    acc
                } else {
                    [acc, vec![number_opt.unwrap()]].concat()
                }
            });

        if numbers.len() < 3 {
            panic!("error lors de la creation de l'instructions");
        }

        Instruction {
            r#move: numbers[0],
            from: numbers[1],
            to: numbers[2]
        }
    }

    fn modif(&self, piles: &Vec<Pile>) -> Vec<Pile> {
        let quantite = self.r#move;
        let dep: usize = self.from as usize;
        let destination: usize = self.to as usize;

        (0..quantite)
            .fold(piles.clone(), |mut acc, current| {
                let depart: Pile = acc[dep - 1].clone();
                let last = depart.out().unwrap_or('?');
                let nouvelle_dest = acc[destination - 1].add(last);
                let nouveau_depart = depart.drop_last();
                acc[dep - 1] = nouveau_depart;
                acc[destination - 1] = nouvelle_dest;
                acc
            })

    }
}