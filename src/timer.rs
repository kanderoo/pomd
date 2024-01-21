use std::{time::{Instant, Duration}, fmt};

pub struct Timer {
    length: Duration,
    start: Instant
}

impl Timer {
    pub fn new(length: Duration) -> Self{
        Timer {
            length,
            start: Instant::now()
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.start.elapsed().as_secs() >= self.length.as_secs()
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let elapsed_minutes: u64 = self.start.elapsed().as_secs() / 60;
        let elapsed_seconds: u64 = self.start.elapsed().as_secs() % 60;

        let length_minutes: u64 = self.start.elapsed().as_secs() / 60;
        let length_seconds: u64 = self.start.elapsed().as_secs() % 60;

        write!(f, "{:02}:{:02}/{:02}:{:02}",
            elapsed_minutes,
            elapsed_seconds,
            length_minutes,
            length_seconds
        )
    }
}