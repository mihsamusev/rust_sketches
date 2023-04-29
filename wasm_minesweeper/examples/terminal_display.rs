use std::collections::HashSet;
use wasm_minesweeper::Board;

fn main() {
    let mines = HashSet::from([(3, 3)]);
    let mut board = Board::with_mines(5, 5, mines);
    board.open((0, 0));
    board.flag((2, 2));

    println!("{}", board);
}
