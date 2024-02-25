use std::time::Duration;

pub struct Config {
    pub work_duration: Duration,
    pub short_break_duration: Duration,
    pub long_break_duration: Duration,
}

impl Default for Config {
    fn default() -> Self  {
        Config {
            work_duration: Duration::from_secs(60 * 25),
            short_break_duration: Duration::from_secs(60 * 5),
            long_break_duration: Duration::from_secs(60 * 15),
        }
    }
}