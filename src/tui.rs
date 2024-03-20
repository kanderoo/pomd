use std::io::{Error, Stdin, Stdout, Write};
use crossterm::{cursor, execute, terminal::{self, disable_raw_mode, enable_raw_mode}};

use crate::app::{App, Mode, PomodoroPhase};

pub struct Tui {
    stdout: Stdout,
    stdin: Stdin,
}

fn get_x_offset(midpoint: (u16, u16), message: &str) -> u16 {
    midpoint.0 - (message.chars().count() / 2) as u16
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
        let terminal_size = terminal::size()?;
        let midpoint = (terminal_size.0 / 2, terminal_size.1 / 2);


        // write state
        let (symbol, status_message) = match app.pom_phase {
            PomodoroPhase::Work => ("🍅 ",  format!("Working ({}/{})", app.pom_count, app.config.poms_till_long_break)),
            PomodoroPhase::ShortBreak => ("☕ ", "Short Break".to_string()),
            PomodoroPhase::LongBreak => ("😴 ", "Long Break".to_string())
        };

        // write if paused
        execute!(self.stdout, cursor::MoveTo(get_x_offset(midpoint, &symbol) + 1, midpoint.1 - 2))?;
        print!("{}", symbol);

        execute!(self.stdout, cursor::MoveTo(get_x_offset(midpoint, &status_message), midpoint.1 - 1))?;
        print!("{}", status_message);

        let mut time_message = format!("{:02}:{:02}", app.remaining_time.as_secs() / 60, app.remaining_time.as_secs() % 60);
        if app.is_paused() {
            time_message = "⏸️  ".to_string() + &time_message;
        }
        execute!(self.stdout, cursor::MoveTo(get_x_offset(midpoint, &time_message), midpoint.1))?;
        print!("{}", time_message);

        if app.config.display_help_line {
            let help_message = "Press [q] to quit, [space] to pause/unpause, [>] to skip current phase, [<] to restart phase";
            execute!(self.stdout, cursor::MoveTo(get_x_offset(midpoint, help_message), midpoint.1 + 1))?;
            print!("{}", help_message);
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
    
    pub fn cleanup(&mut self) -> Result<(), Error> {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        execute!(self.stdout, cursor::MoveTo(0,0))?;
        self.exit_rawmode()?;
        
        Ok(())
    }
}