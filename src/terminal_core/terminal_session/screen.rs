use crate::terminal_core::terminal_session::cell::Cell;

pub struct Screen {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            cells.push(Cell::default());
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, row: usize, column: usize) -> &Cell {
        &self.cells[row * self.width + column]
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> &mut Cell {
        &mut self.cells[row * self.width + column]
    }
}
