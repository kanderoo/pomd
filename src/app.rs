use std::io::Error;
use std::time::Duration;

use crate::config::Config;
use crate::log::Logger;
use crate::notification::send_notification;

pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak
}

pub enum Mode {
    /// Standard mode where the timer is shown
    Timer,
    /// User input mode
    Form
}

pub struct App {
    pub mode: Mode,
    pub remaining_time: Duration,
    pub pom_phase: PomodoroPhase,
    pub pom_count: u8,
    pub paused: bool,
    pub config: Config,
    pub logger: Logger,
    quit_flag: bool
}

impl App {
    pub fn new(config: Config, logger: Logger) -> Self {
        Self {
            mode: Mode::Timer,
            remaining_time: config.work_duration,
            pom_count: 1,
            pom_phase: PomodoroPhase::Work,
            paused: true,
            config,
            logger,
            quit_flag: false
        }
    }

    pub fn decrement_timer(&mut self) {
        if !self.paused {
            self.remaining_time -= Duration::from_secs(1);
            if self.remaining_time.is_zero() {
                self.next_phase();
            }
        }
    }

    pub fn next_phase(&mut self) {
        // there's a whole lotta "self" going on here, not sure if there's a syntax shortcut I'm missing out on
        match self.pom_phase {
            PomodoroPhase::Work => {
                if self.pom_count == self.config.poms_till_long_break {
                    self.pom_phase = PomodoroPhase::LongBreak;
                    self.remaining_time = self.config.long_break_duration;
                } else {
                    self.pom_phase = PomodoroPhase::ShortBreak;
                    self.remaining_time = self.config.short_break_duration;
                }
                self.pom_count += 1;
                // move to form mode to ask user what they accomplished
                self.mode = Mode::Form;
            },
            PomodoroPhase::ShortBreak => {
                self.pom_phase = PomodoroPhase::Work;
                self.remaining_time = self.config.work_duration;
            },
            PomodoroPhase::LongBreak => {
                self.pom_phase = PomodoroPhase::Work;
                self.remaining_time = self.config.work_duration;
                self.pom_count = 1;
            }
        }
        if self.config.notifications {
            match send_notification(&self.pom_phase) {
                Ok(_) => (), Err(e) => eprintln!("Can't send a notification: {}", e)
            }
        }
        self.paused = true;
    }
    
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
    
    pub fn reset_phase_timer(&mut self) {
        match self.pom_phase {
            PomodoroPhase::Work => self.remaining_time = self.config.work_duration,
            PomodoroPhase::ShortBreak => self.remaining_time = self.config.short_break_duration,
            PomodoroPhase::LongBreak => self.remaining_time = self.config.long_break_duration,
        }
    }
    
    pub fn log(&mut self, description: &str) -> Result<(), Error> {
        self.logger.log_pomodoro(description)
    }
    
    pub fn quit(&mut self) {
        self.quit_flag = true;
    }
    
    pub fn should_quit(&self) -> bool {
        self.quit_flag
    }
}