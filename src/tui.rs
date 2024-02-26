use std::io::{Stdout, Write};
use crossterm::{cursor, execute, terminal::{self, disable_raw_mode, enable_raw_mode}};

use crate::app::{App, PomodoroPhase};

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

    pub fn display_tui(&mut self, app: &App) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        execute!(self.stdout, cursor::MoveTo(0, 0)).unwrap();

        // write if paused
        if app.paused {
            print!("[ Paused ] ");        
        }

        execute!(self.stdout, cursor::MoveTo(0, 1)).unwrap();
        // write state
        match app.pom_phase {
            PomodoroPhase::Work => print!("🍅  Working ({}/{})", app.pom_count, app.config.poms_till_long_break),
            PomodoroPhase::ShortBreak => print!("☕  Short Break"),
            PomodoroPhase::LongBreak => print!("😴  Long Break")
        };
        
        execute!(self.stdout, cursor::MoveTo(0, 2)).unwrap();
        print!("{:02}:{:02}", app.remaining_time.as_secs() / 60, app.remaining_time.as_secs() % 60);

        execute!(self.stdout, cursor::MoveTo(0, 3)).unwrap();
        print!("Press [q] to quit, [space] to pause/unpause, [>] to skip current phase, [<] to restart phase");

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