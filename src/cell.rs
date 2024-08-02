use rand::Rng;

pub struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new(living_chance: f64) -> Self {
        Self {
            alive: rand::thread_rng().gen_bool(living_chance),
        }
    }

    pub fn to_string(&self) -> String {
        if self.alive {
            String::from("x")
        } else {
            String::from(".")
        }
    }

    pub fn update_alive_status(&mut self) {
        self.alive = !self.alive;
    }
}
