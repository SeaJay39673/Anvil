pub struct Cursor {
    row: usize,
    column: usize,
}

impl Cursor {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}
