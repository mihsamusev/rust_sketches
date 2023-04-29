use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        Universe {
            width: width,
            height: height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn live_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter() {
            for delta_col in [self.width - 1, 0, 1].iter() {
                // dont count iself
                if *delta_row == 0 && *delta_col == 0 {
                    continue;
                }

                // neigbours with wrapping
                let row_near = (row + delta_row) % self.height;
                let col_near = (col + delta_col) % self.width;
                let idx = self.get_index(row_near, col_near);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let n_live_neighbours = self.live_count(row, col);

                let next_cell = match (cell, n_live_neighbours) {
                    // under population
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // birth
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    // remain same
                    (cell_state, _) => cell_state,
                };
                next_cells[idx] = next_cell;
            }
        }
        self.cells = next_cells;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.chunks(self.width as usize) {
            for &cell in line {
                if cell == Cell::Dead {
                    write!(f, "{}", '◻')?;
                } else {
                    write!(f, "{}", '◼')?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
fn main() {
    let mut universe = Universe::new(5, 5);
    universe.cells[11] = Cell::Alive;
    universe.cells[12] = Cell::Alive;
    universe.cells[13] = Cell::Alive;
    println!("|<Tick 0:|\n{}", &universe);
    let mut i = 0;
    while i < 100 {
        universe.tick();
        println!("|<Tick {i}:|\n{}", &universe);
        i += 1;
    }
}
