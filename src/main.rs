mod cell;

use crate::cell::Cell;

fn main() {
    let my_cell: Cell = Cell { alive: false };

    println!("{}", my_cell.to_string());
}
