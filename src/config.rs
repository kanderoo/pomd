use std::time::Duration;

use crate::cli::Args;

pub struct Config {
    pub work_duration: Duration,
    pub short_break_duration: Duration,
    pub long_break_duration: Duration,
    pub poms_till_long_break: u8
}

impl Config {
    // this is kinda terrible, there's probably a better way to do this.
    pub fn integrate_args(&mut self, args: Args) {
        match args.pomodoro_length {
            Some(length) => self.work_duration = Duration::from_secs(length as u64 * 60),
            None => ()
        }

        match args.short_break_length {
            Some(length) => self.short_break_duration = Duration::from_secs(length as u64 * 60),
            None => ()
        }

        match args.long_break_length {
            Some(length) => self.long_break_duration = Duration::from_secs(length as u64 * 60),
            None => ()
        }

        match args.sequence_count {
            Some(count) => self.poms_till_long_break = count,
            None => ()
        }
    }
}

impl Default for Config {
    fn default() -> Self  {
        Config {
            work_duration: Duration::from_secs(60 * 25),
            short_break_duration: Duration::from_secs(60 * 5),
            long_break_duration: Duration::from_secs(60 * 30),
            poms_till_long_break: 4
        }
    }
}