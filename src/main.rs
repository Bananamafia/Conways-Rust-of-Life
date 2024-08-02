mod cell;

use crate::cell::Cell;

fn main() {
    let living_chance: f64 = 0.33;

    let mut my_cell: Cell = Cell::new(living_chance);

    println!("{}", my_cell.to_string());

    my_cell.update_alive_status();

    println!("{}", my_cell.to_string());

    my_cell.update_alive_status();

    println!("{}", my_cell.to_string());
}
