use std::time::Duration;

use clap::ValueEnum;

use crate::args::Args;

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

#[derive(ValueEnum, Clone)]
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
    pub fn from_args(args: Args) -> Self {
        let mut config = Self{
            logging: args.log,
            notifications: !args.disable_notifications,
            ..Default::default()
        };

        match args.pomodoro_length {
            Some(length) => config.work_duration = Duration::from_secs(length as u64 * 60), None => ()
        }

        match args.short_break_length {
            Some(length) => config.short_break_duration = Duration::from_secs(length as u64 * 60), None => ()
        }

        match args.long_break_length {
            Some(length) => config.long_break_duration = Duration::from_secs(length as u64 * 60), None => ()
        }

        match args.sequence_count {
            Some(count) => config.poms_till_long_break = count, None => ()
        }

        match args.pause_behavior {
            Some(behavior) => config.pause_behavior = behavior, None => ()
        }
        
        config
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