use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::time::Duration;

use chrono::Local;

use crate::config::Config;
use crate::notification::{send_phase_notification, send_reminder};
use crate::sound::SoundHandler;
use crate::config::PauseBehavior;

#[derive(PartialEq)]
pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak
}

#[derive(PartialEq)]
pub enum Mode {
    /// Standard mode where the timer is shown
    Timer,
    /// User input mode
    Form
}

pub struct App {
    pub mode: Mode,
    pub remaining_time: Duration,
    pub paused_time: Duration,
    pub pom_phase: PomodoroPhase,
    pub pom_count: u8,
    paused: bool,
    pub config: Config,
    sound_handler: SoundHandler,
    quit_flag: bool
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            mode: Mode::Timer,
            remaining_time: config.work_duration,
            pom_count: 1,
            pom_phase: PomodoroPhase::Work,
            paused: true,
            config,
            sound_handler: SoundHandler::new(),
            paused_time: Duration::ZERO,
            quit_flag: false
        }
    }

    fn change_phase(&mut self) {
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
                if self.config.logging {
                    self.mode = Mode::Form;
                }
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
    }

    pub fn handle_tick(&mut self) {
        // check if timer should decrement
        if self.paused {
            self.paused_time += Duration::from_secs(1);
            // send a reminder every 10 minutes
            if self.paused_time.as_secs() % (10 * 60) == 0 {
                send_reminder(&self.paused_time);
            }
        } else if self.mode == Mode::Timer {
            self.remaining_time -= Duration::from_secs(1);
            if self.remaining_time.is_zero() {
                self.natural_next_phase();
            }
        }
    }

    fn should_pause(&self) -> bool {
        // if you're in the form view, you'd better pause
        if self.mode == Mode::Form {
            return true
        }

        match self.config.pause_behavior {
            PauseBehavior::Always => true,
            PauseBehavior::Never => false,
            PauseBehavior::OnWork => self.pom_phase == PomodoroPhase::Work,
            PauseBehavior::OnBreak => self.pom_phase != PomodoroPhase::Work
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    fn pause(&mut self) {
        self.paused = true;
        self.paused_time = Duration::ZERO;
    }

    // Move to the next phase, assumes skip button pressed
    // Will always pause, won't play a sound
    pub fn skip_next_phase(&mut self) {
        // change the state
        self.change_phase();

        // always pause
        self.pause();
    }

    // Move to the next phase, assumes _timer rollover_
    pub fn natural_next_phase(&mut self) {
        // change the state
        self.change_phase();

        // play a sound
        self.sound_handler.play_complete().unwrap();

        // maybe pause
        if self.should_pause() {
            self.pause();
        }

        // notify
        if self.config.notifications {
            send_phase_notification(&self.pom_phase);
        }
    }
    
    pub fn toggle_pause(&mut self) {
        if !self.paused {
            self.pause()
        } else {
            self.paused = false;
        }
    }
    
    pub fn reset_phase_timer(&mut self) {
        match self.pom_phase {
            PomodoroPhase::Work => self.remaining_time = self.config.work_duration,
            PomodoroPhase::ShortBreak => self.remaining_time = self.config.short_break_duration,
            PomodoroPhase::LongBreak => self.remaining_time = self.config.long_break_duration,
        }
        self.paused = true;
    }
    
    pub fn log(&mut self, description: &str) -> Result<(), Error> {
        let mut file = OpenOptions::new().append(true).create(true).open(&self.config.log_filepath)?;

        let now = Local::now();

        writeln!(file, "{}: {}", now, description)?;

        Ok(())
    }
    
    pub fn quit(&mut self) {
        self.quit_flag = true;
    }
    
    pub fn should_quit(&self) -> bool {
        self.quit_flag
    }
}