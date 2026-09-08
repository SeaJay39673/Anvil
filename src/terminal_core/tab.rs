use crate::terminal_core::pane::{PaneId, PaneNode};

pub struct Tab {
    panes: PaneNode,
    focused_pane: PaneId,
}
