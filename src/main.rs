use core::time;
use std::io::stdout;
use ratatui::Terminal;

use clap::Parser;
use crossterm::terminal::enable_raw_mode;
use ratatui::backend::CrosstermBackend;
use timer::Timer;
use ui::draw_ui;

mod cli;
mod timer;
mod ui;

pub enum State {
    Pomdodoro,
    ShortBreak,
    LongBreak
}

fn main() {
    let args = cli::Args::parse();

    // 25 minute long timer
    let timer = Timer::new(time::Duration::from_secs(25 * 60));
    
    draw_ui().expect("Could not draw ui");
}