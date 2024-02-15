mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Default for Universe {
    fn default() -> Self {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Self::new(width, height, cells)
    }
}

impl Universe {
    pub fn new(width: u32, height: u32, cells: Vec<Cell>) -> Self {
        assert!(width * height == cells.len() as u32);
        Self {
            width,
            height,
            cells,
        }
    }
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // We use self.height - 1 instead of just -1 so we wrap around the grid
        let row_deltas = vec![self.height - 1, 0, 1];
        let col_deltas = vec![self.width - 1, 0, 1];
        for delta_r in row_deltas.iter() {
            for delta_c in col_deltas.iter() {
                if *delta_r == 0 && *delta_c == 0 {
                    // Exclude the actual cell
                    continue;
                }
                let idx = self.get_index(
                    (row + delta_r) % self.height,
                    (column + delta_c) % self.width,
                );
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    pub fn len(&self) -> usize {
        self.cells.len()
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let neighbor_count = self.live_neighbor_count(row, col);
                let next_cell = match (cell, neighbor_count) {
                    // Underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Reproduction
                    (Cell::Dead, 3) => Cell::Alive,
                    // Lives on
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // All other cells remain as-is
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub fn greet(input: &str) {
    let mut s = "Hello, wasm-game-of-life! ".to_owned();
    s.push_str(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let uni = Universe::default();
        assert_eq!(64 * 64, uni.len());
    }
}
