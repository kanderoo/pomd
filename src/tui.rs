use std::io::Stdout;
use crossterm::{execute, terminal};

use crate::{PomodoroPhase, State};

pub struct Tui {
    stdout: Stdout
}

impl Tui {
    pub fn new(stdout: Stdout) -> Self {
        Self {
            stdout
        }
    }

    pub fn display_tui(&mut self, state: &State) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All));

        // write if paused
        if state.paused {
            print!("[ Paused ] ");        
        }
        // write state
        match state.pom_phase {
            PomodoroPhase::Work => println!("Working:"),
            PomodoroPhase::Break => println!("Short Break:"),
            PomodoroPhase::LongBreak => println!("Long Break:")
        };
        
        println!("{}:{}", state.remaining_time.as_secs() / 60, state.remaining_time.as_secs() % 60);
    }
}
