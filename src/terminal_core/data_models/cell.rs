pub struct Cell {
    pub character: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { character: ' ' }
    }
}
