use rand::Rng;

#[derive(Clone, Copy)]
pub struct Cell {
    alive: bool,
    living_neighbor_count: u8,
}

impl Cell {
    pub fn new(living_chance: f64) -> Self {
        Self {
            alive: rand::thread_rng().gen_bool(living_chance),
            living_neighbor_count: 0,
        }
    }

    pub fn to_string(&self) -> &str {
        if self.alive {
            "██"
        } else {
            "  "
        }
    }

    pub fn update_living_neighbor_count(&mut self, neighbors: Vec<Cell>) {
        self.living_neighbor_count = neighbors.iter().filter(|cell| cell.alive).count() as u8
    }

    pub fn update_alive_status(&mut self) {
        match self.living_neighbor_count {
            0 | 1 => self.alive = false,
            2 => self.alive = self.alive,
            3 => self.alive = true,
            4.. => self.alive = false,
        }
    }

}
