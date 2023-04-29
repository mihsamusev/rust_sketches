pub mod minesweeper;
pub use minesweeper::Board;

use wasm_bindgen::prelude::*;

impl Board {}
#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    let ms = Board::new(10, 10, 5);
    ms.to_string()
}

#[wasm_bindgen(js_name = openCell)]
pub fn open_cell(x: i32, y: i32) {}

#[wasm_bindgen(js_name = flagCell)]
pub fn flag_cell(x: i32, y: i32) {}
