use std::io::{Stdout, Write};
use crossterm::{cursor, execute, terminal::{self, disable_raw_mode, enable_raw_mode}};

use crate::{config::Config, PomodoroPhase, State};

pub struct Tui {
    stdout: Stdout
}

impl Tui {
    pub fn new(stdout: Stdout) -> Self {
        enable_raw_mode().expect("Cannot enable raw mode");
        execute!(&stdout, cursor::Hide).unwrap();
        Self {
            stdout
        }
    }

    pub fn display_tui(&mut self, state: &State, config: &Config) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        execute!(self.stdout, cursor::MoveTo(0, 0)).unwrap();

        // write if paused
        if state.paused {
            print!("[ Paused ] ");        
        }

        execute!(self.stdout, cursor::MoveTo(0, 1)).unwrap();
        // write state
        match state.pom_phase {
            PomodoroPhase::Work => print!("🍅 Working ({}/{})", state.pom_count, config.poms_till_long_break),
            PomodoroPhase::Break => print!("☕ Short Break"),
            PomodoroPhase::LongBreak => print!("😴 Long Break")
        };
        
        execute!(self.stdout, cursor::MoveTo(0, 2)).unwrap();
        print!("{:02}:{:02}", state.remaining_time.as_secs() / 60, state.remaining_time.as_secs() % 60);

        execute!(self.stdout, cursor::MoveTo(0, 3)).unwrap();
        print!("Press [q] to quit, [space] to pause/unpause, [>] to skip current phase, [<] to restart timer");

        self.stdout.flush().unwrap();
    }
    
    pub fn cleanup(&self) {
        execute!(&self.stdout, cursor::Show).unwrap();
        execute!(&self.stdout, cursor::MoveToNextLine(1)).unwrap();
        print!("Goodbye!");
        execute!(&self.stdout, cursor::MoveToNextLine(1)).unwrap();
        disable_raw_mode().expect("Cannot disable raw mode");
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.cleanup();
    }
}
