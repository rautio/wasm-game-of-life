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

impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    pub fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
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
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
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
        Self {
            width,
            height,
            cells,
        }
    }
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
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
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
