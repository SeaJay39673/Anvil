use crate::terminal_core::terminal_session::{cursor::Cursor, screen::Screen};

pub struct Session {
    screen: Screen,
    cursor: Cursor,
}

impl Session {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            screen: Screen::new(width, height),
            cursor: Cursor::new(0, 0),
        }
    }
}
