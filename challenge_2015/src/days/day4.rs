use crate::days::day::{ChargementInput, Day};
// use crate::crypto_tools::md5::MD5;
pub struct Day4;

impl ChargementInput for Day4 {}

impl Day4 {
    /*
    pub fn get_first_line_input(&self) -> String {
        self.input().first().unwrap().to_string()
    }

    pub fn is_valid_5_first_0(&self, hash: &String) -> bool {
        hash[0..5].to_string() == "00000"
    }

    pub fn is_valid_6_first_0(&self, hash: &String) -> bool {
        hash[0..6].to_string() == "000000"
    }
     */
}

impl Day for Day4 {
    fn day(&self) -> u8 { 4u8 }

    fn response_1(&self) -> String {
        // let hash = self.get_first_line_input().md5();
        /*
        (0..999999)
            .find(|i| self.is_valid_5_first_0(&format!("{}{}", self.get_first_line_input(), i).md5()))
            .unwrap()
            .to_string()

         */

        "long time response".to_string()
    }

    fn response_2(&self) -> String {
        /*
        (0..9999999)
            .find(|i| self.is_valid_6_first_0(&format!("{}{}", self.get_first_line_input(), i).md5()))
            .unwrap()
            .to_string()
         */

        "long time response".to_string()
    }
}