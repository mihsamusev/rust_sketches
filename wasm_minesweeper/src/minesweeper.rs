use rand::Rng;
use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

type Position = (i32, i32);

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    open_cells: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_cells: HashSet<Position>,
    game_over: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpenResult {
    None,
    Mine,
    NoMine(u8),
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                let pos = (i as i32, j as i32);
                if self.open_cells.contains(&pos) {
                    write!(f, " {} ", self.neighbour_mines(pos))?;
                } else if self.flagged_cells.contains(&pos) {
                    f.write_str("🚩 ")?;
                } else if self.mines.contains(&(pos)) {
                    f.write_str("💥 ")?; // or 💣
                } else {
                    f.write_str("🟧 ")?;
                };
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut mines = HashSet::new();
        while mines.len() < mine_count {
            let x = rng.gen_range(0..width as i32);
            let y = rng.gen_range(0..height as i32);
            let mine_pos = (x, y);
            if let None = mines.get(&mine_pos) {
                mines.insert(mine_pos);
            }
        }

        Board {
            width,
            height,
            open_cells: HashSet::new(),
            mines,
            flagged_cells: HashSet::new(),
            game_over: false,
        }
    }

    pub fn with_mines(width: usize, height: usize, mines: HashSet<Position>) -> Self {
        Board {
            width,
            height,
            open_cells: HashSet::new(),
            mines,
            flagged_cells: HashSet::new(),
            game_over: false,
        }
    }

    fn iter_neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width as i32;
        let height = self.height as i32;
        ((x - 1).max(0)..=(x + 1).min(width - 1))
            .flat_map(move |i| ((y - 1).max(0)..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&p| p != (x, y))
    }

    fn iter_openable_neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width as i32;
        let height = self.height as i32;
        let open_cells = self.open_cells.clone();
        let flagged_cells = self.flagged_cells.clone();

        ((x - 1).max(0)..=(x + 1).min(width - 1))
            .flat_map(move |i| ((y - 1).max(0)..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&p| p != (x, y))
            .filter(move |p| !open_cells.contains(p))
            .filter(move |p| !flagged_cells.contains(p))
    }

    fn neighbour_mines(&self, pos: Position) -> u8 {
        self.iter_neighbours(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open_one(&mut self, pos: Position) -> OpenResult {
        if self.game_over || self.flagged_cells.contains(&pos) {
            return OpenResult::None;
        }
        self.open_cells.insert(pos);

        if let Some(_) = self.mines.get(&pos) {
            OpenResult::Mine
        } else {
            let mines_close = self.neighbour_mines(pos);
            OpenResult::NoMine(mines_close)
        }
    }

    pub fn open(&mut self, pos: Position) -> OpenResult {
        if self.game_over || self.flagged_cells.contains(&pos) {
            return OpenResult::None;
        }
        self.open_cells.insert(pos);

        if let Some(_) = self.mines.get(&pos) {
            OpenResult::Mine
        } else {
            let mines_close = self.neighbour_mines(pos);
            if mines_close == 0 {
                for neigh_pos in self.iter_openable_neighbours(pos) {
                    self.open(neigh_pos);
                }
            }
            OpenResult::NoMine(mines_close)
        }
    }

    pub fn flag(&mut self, pos: Position) {
        if !self.game_over {
            if self.flagged_cells.contains(&pos) & !self.open_cells.contains(&pos) {
                self.flagged_cells.remove(&pos);
            } else {
                self.flagged_cells.insert(pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let ms = Board::new(10, 15, 5);
        assert_eq!(ms.mines.len(), 5);
    }

    #[test]
    fn test_open_mine() {
        let mines = HashSet::from([(3, 3)]);
        let mut ms = Board::with_mines(4, 5, mines);
        let result = ms.open((3, 3));
        assert_eq!(result, OpenResult::Mine)
    }

    #[test]
    fn test_open_next_to_one_mine() {
        let mines = HashSet::from([(0, 0), (3, 3)]);
        let mut ms = Board::with_mines(4, 5, mines);
        let result = ms.open((1, 1));
        assert_eq!(result, OpenResult::NoMine(1))
    }

    #[test]
    fn test_open_next_to_two_mines() {
        let mines = HashSet::from([(0, 0), (2, 2), (3, 3)]);
        let mut ms = Board::with_mines(4, 5, mines);
        let result = ms.open((1, 1));
        assert_eq!(result, OpenResult::NoMine(2))
    }

    #[test]
    fn test_neighbours() {
        let ms = Board::new(4, 5, 0);

        // upper left corner
        let mut result: Vec<Position> = ms.iter_neighbours((0, 0)).collect();
        result.sort();
        assert_eq!(result, vec![(0, 1), (1, 0), (1, 1)]);

        // lower right corner
        let mut result: Vec<Position> = ms.iter_neighbours((3, 4)).collect();
        result.sort();
        assert_eq!(result, vec![(2, 3), (2, 4), (3, 3)]);

        // mid board
        let mut result: Vec<Position> = ms.iter_neighbours((1, 1)).collect();
        result.sort();
        assert_eq!(
            result,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
    }

    #[test]
    fn test_closed_neigbours() {
        let mut ms = Board::new(4, 4, 0);
        ms.open_one((0, 0));
        ms.flag((2, 2));

        // upper left corner
        let mut result: Vec<Position> = ms.iter_openable_neighbours((1, 1)).collect();
        result.sort();
        assert_eq!(result, vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]);
    }
}
