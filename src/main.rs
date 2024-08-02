mod cell;

use crate::cell::Cell;

fn main() {
    let living_chance: f64 = 0.2;
    let board_height: u32 = 20;
    let board_width: u32 = 40;

    let cells: Vec<Vec<Cell>> = fill_board(living_chance, board_height, board_width);

    print_cells(&cells);
}

fn fill_board(living_chance: f64, height: u32, width: u32) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Vec<Cell>> = vec![];

    for _ in 0..height {
        let mut row: Vec<Cell> = vec![];
        for _ in 0..width {
            row.push(Cell::new(living_chance));
        }
        cells.push(row);
    }

    cells
}

fn print_cells(cells: &Vec<Vec<Cell>>) {
    for row in cells {
        for column in row {
            print!("{}", column.to_string())
        }
        println!();
    }
}
