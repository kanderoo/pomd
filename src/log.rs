use std::{fs::{File, OpenOptions}, io::{Error, Write}};

use chrono::Local;

pub struct Logger {
    log_file: File
}

impl Logger {
    pub fn new(filepath: &str) -> Result<Self, Error> {
        let file = OpenOptions::new().append(true).create(true).open(filepath)?;

        Ok(Self {
            log_file: file
        })
    }

    pub fn log_pomodoro(&self, description: &str) -> Result<(), Error> {
        let now = Local::now();

        writeln!(&self.log_file, "{}: {}", now, description)?;

        Ok(())
    }
}
