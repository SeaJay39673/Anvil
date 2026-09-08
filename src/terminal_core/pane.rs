use std::sync::{LazyLock, Mutex};

use crate::terminal_core::terminal_session::Session;

pub type PaneId = u64;

static NEXT_ID: LazyLock<Mutex<PaneId>> = LazyLock::new(|| Mutex::new(0));

#[derive(Debug, Clone, Copy)]
pub struct PaneGeometry {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl PaneGeometry {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub enum PaneNode {
    Split {
        orientation: Orientation,
        first: Box<Pane>,
        second: Box<Pane>,
    },
    Session(Session),
}

pub struct Pane {
    pub id: PaneId,
    pub geometry: PaneGeometry,
    pub node: PaneNode,
}

impl Pane {
    pub fn new(id: PaneId, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            id,
            geometry: PaneGeometry::new(x, y, width, height),
            node: PaneNode::Session(Session::new(width, height)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_to_pane() {
        let mut pane = Pane::new(0, 0, 0, 100, 100);
    }
}
