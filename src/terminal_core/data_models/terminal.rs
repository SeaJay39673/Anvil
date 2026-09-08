use crate::terminal_core::data_models::{cursor::Cursor, screen::Screen};

pub struct Terminal {
    screen: Screen,
    cursor: Cursor,
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            screen: Screen::new(width, height),
            cursor: Cursor::new(0, 0),
        }
    }
}
