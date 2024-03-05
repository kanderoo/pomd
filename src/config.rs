use std::time::Duration;
use serde::{Serialize, Deserialize};

use clap::ValueEnum;

use crate::args::Args;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub work_duration: Duration,
    pub short_break_duration: Duration,
    pub long_break_duration: Duration,
    pub poms_till_long_break: u8,
    pub notifications: bool,
    pub log_filepath: String,
    pub logging: bool,
    pub pause_behavior: PauseBehavior
}

#[derive(Serialize, Deserialize, ValueEnum, Clone)]
pub enum PauseBehavior {
    /// Autostart every timer
    Never,
    /// Pause at the start of pomodoros
    OnWork,
    /// Pause at the start of breaks
    OnBreak,
    /// Pause at the start of every timer
    Always
}

impl Config {
    // this is kinda terrible, there's probably a better way to do this.
    pub fn integrate_args(&mut self, args: Args) {
        if let Some(length) = args.pomodoro_length {
            self.work_duration = Duration::from_secs(length as u64 * 60);
        }

        if let Some(length) = args.short_break_length {
            self.short_break_duration = Duration::from_secs(length as u64 * 60);
        }

        if let Some(length) = args.long_break_length {
            self.long_break_duration = Duration::from_secs(length as u64 * 60);
        }

        if let Some(count) =  args.sequence_count {
            self.poms_till_long_break = count;
        }

        if let Some(behavior) =  args.pause_behavior {
            self.pause_behavior = behavior;
        }
    }
}

impl Default for Config {
    fn default() -> Self  {
        Config {
            work_duration: Duration::from_secs(60 * 25),
            short_break_duration: Duration::from_secs(60 * 5),
            long_break_duration: Duration::from_secs(60 * 30),
            poms_till_long_break: 4,
            notifications: true,
            log_filepath: "pomodoros.log".to_string(),
            logging: false,
            pause_behavior: PauseBehavior::OnWork
        }
    }
}