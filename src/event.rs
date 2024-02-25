use std::sync::mpsc::Receiver;

use crossterm::event::{KeyCode, KeyEvent};

pub enum Event {
    TimerTick,
    KeyEvent(KeyEvent)
}

pub struct EventHandler {
    rx: Receiver<Event>
}

impl EventHandler {
    pub fn new(rx: Receiver<Event>) -> Self {
        Self {
            rx
        }
    }

    // in the future, this should be async / running on another thread
    pub fn poll_event(&self) -> Event {
        self.rx.recv().unwrap()
    }
}