use std::{sync::mpsc::Sender, thread::{self, JoinHandle}};
use crossterm::event::read;

use crate::event::Event;

pub struct KeyListener {
    tx: Sender<Event>
}

impl KeyListener {
    pub fn new(tx: Sender<Event>) -> Self {
        Self {
            tx
        }
    }
    
    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match read().unwrap() {
                    crossterm::event::Event::Key(event) => self.tx.send(Event::KeyEvent(event)).unwrap(),
                    _ => ()
                }
            }
        })
    }
}
