pub struct Cell {
    pub alive: bool,
}

impl Cell {
    pub fn to_string(&self) -> String {
        if self.alive {
            String::from("x")
        } else {
            String::from(" ")
        }
    }
}
