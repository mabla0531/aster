use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::Stdout;

pub fn log(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    loop {}
}
