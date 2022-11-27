trait Hexa {
    fn to_hexa(&self) -> String;
}

impl Hexa for String {
    fn to_hexa(&self) -> String {
        self
            .as_bytes()
            .iter()
            .map(|c| format!("{:x}", c))
            .collect::<Vec<String>>()
            .join("")
    }
}