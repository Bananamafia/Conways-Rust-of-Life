mod cell;

use std::{process::Command, thread, time::Duration};

use crate::cell::Cell;

fn main() {
    let living_chance: f64 = 0.2;
    let board_height: u16 = 60;
    let board_width: u16 = 120;

    let mut cells: Vec<Vec<Cell>> = fill_board(living_chance, board_height, board_width);

    loop {
        clear_console();
        print_cells(&cells);
        do_live_cycle(&mut cells);
        thread::sleep(Duration::from_millis(150));
    }
}

fn fill_board(living_chance: f64, height: u16, width: u16) -> Vec<Vec<Cell>> {
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

fn do_live_cycle(board: &mut Vec<Vec<Cell>>) {
    let board_height: usize = board.len();
    let board_width: usize = board[0].len();

    for y in 0..board_height {
        for x in 0..board_width {
            let upper_row: usize = get_next_lower(y, board_height - 1);
            let lower_row: usize = get_next_higher(y, board_height - 1);
            let left_column: usize = get_next_lower(x, board_width - 1);
            let right_column: usize = get_next_higher(x, board_width - 1);

            let mut neighbors: Vec<Cell> = vec![];

            neighbors.push(board[upper_row][left_column]);
            neighbors.push(board[upper_row][x]);
            neighbors.push(board[upper_row][right_column]);

            neighbors.push(board[y][left_column]);
            neighbors.push(board[y][right_column]);

            neighbors.push(board[lower_row][left_column]);
            neighbors.push(board[lower_row][x]);
            neighbors.push(board[lower_row][right_column]);

            let current_cell = &mut board[y][x];
            current_cell.update_living_neighbor_count(neighbors);
        }
    }

    for y in 0..board_height {
        for x in 0..board_width {
            let current_cell = &mut board[y][x];
            current_cell.update_alive_status();
        }
    }
}

fn get_next_lower(current_value: usize, max_value: usize) -> usize {
    if current_value > 0 {
        current_value - 1
    } else {
        max_value
    }
}

fn get_next_higher(current_value: usize, max_value: usize) -> usize {
    if current_value < max_value {
        current_value + 1
    } else {
        0
    }
}

fn print_cells(cells: &Vec<Vec<Cell>>) {
    for row in cells {
        for column in row {
            print!("{}", column.to_string())
        }
        println!();
    }
}

fn clear_console() {
    Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
}
