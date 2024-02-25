use std::{io::stdout, sync::mpsc::Sender, thread::{self, sleep, JoinHandle}, time::Duration};

use crate::event::Event;

// waits for an interval, then sends a TimerTick event
pub struct Timer {
    interval: Duration,
    tx: Sender<Event>,
}

impl Timer {
    pub fn new(tx: Sender<Event>, interval: Duration) -> Self {
        Self {
            tx,
            interval
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                sleep(self.interval);
                match self.tx.send(Event::TimerTick) {
                    Ok(x) => x,
                    Err(e) => println!("Could not send timer tick event! {}", e)
                }
            }
        })
    }
}