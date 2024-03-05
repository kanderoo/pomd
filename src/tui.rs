use std::io::{Error, Stdin, Stdout, Write};
use crossterm::{cursor, execute, terminal::{self, disable_raw_mode, enable_raw_mode}};

use crate::app::{App, Mode, PomodoroPhase};

pub struct Tui {
    stdout: Stdout,
    stdin: Stdin,
}

impl Tui {
    pub fn new(stdin: Stdin, stdout: Stdout) -> Result<Self, Error> {
        let mut tui = Self {
            stdout,
            stdin,
        };
        tui.enter_rawmode()?;
        Ok(tui)
    }
    
    pub fn display(&mut self, app: &mut App) -> Result<(), Error> {
        match app.mode {
            Mode::Timer => self.display_timer(app),
            Mode::Form => self.display_form(app)
        }
    }

    fn display_timer(&mut self, app: &App) -> Result<(), Error> {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(self.stdout, cursor::MoveTo(0, 0))?;

        // write if paused
        if app.paused {
            print!("[ Paused ] ");
        }

        execute!(self.stdout, cursor::MoveTo(0, 1))?;
        // write state
        match app.pom_phase {
            PomodoroPhase::Work => print!("🍅 Working ({}/{})", app.pom_count, app.config.poms_till_long_break),
            PomodoroPhase::ShortBreak => print!("☕ Short Break"),
            PomodoroPhase::LongBreak => print!("😴 Long Break")
        };
        
        execute!(self.stdout, cursor::MoveTo(0, 2))?;
        print!("{:02}:{:02}", app.remaining_time.as_secs() / 60, app.remaining_time.as_secs() % 60);

        execute!(self.stdout, cursor::MoveTo(0, 3))?;

        if app.config.display_help_line {
            print!("Press [q] to quit, [space] to pause/unpause, [>] to skip current phase, [<] to restart phase");
        }

        self.stdout.flush()?;
        Ok(())
    }
    
    pub fn display_form(&mut self, app: &mut App) -> Result<(), Error> {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(self.stdout, cursor::MoveTo(0,0))?;
        print!("What did you accomplish during this pomodoro? (Leave blank to skip):");
        self.exit_rawmode()?;

        let mut input = String::new();

        self.stdin.read_line(&mut input)?;
        
        // breaking MVC by accessing app directly?
        if !input.trim().is_empty() {
            app.log(input.trim()).unwrap();
        }
        
        app.mode = Mode::Timer;

        self.enter_rawmode()?;
        
        Ok(())
    }
    
    fn enter_rawmode(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        execute!(self.stdout, cursor::Hide)?;
        Ok(())
    }
    
    fn exit_rawmode(&self) -> Result<(), Error> {
        execute!(&self.stdout, cursor::MoveToNextLine(1))?;
        execute!(&self.stdout, cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }
    
    pub fn cleanup(&self) -> Result<(), Error> {
        self.exit_rawmode()?;
        println!("Goodbye!");
        
        Ok(())
    }
}