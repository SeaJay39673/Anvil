use crate::terminal_core::tab::Tab;

pub struct Terminal {
    width: usize,
    height: usize,
    tabs: Vec<Tab>,
    active_tab: usize,
}
