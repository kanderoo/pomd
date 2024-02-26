use std::sync::mpsc::{Receiver, RecvError};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

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
    
    fn handle_key_event(event: KeyEvent, app: &mut App) {
        match event.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') => {
                if event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            KeyCode::Char('<') => {
                app.reset_phase_timer();
                app.paused = true;
            },
            KeyCode::Char('>') => app.next_phase(),
            KeyCode::Char(' ') => app.toggle_pause(),
            _ => ()
        }
    }

    pub fn handle_event(&self, app: &mut App) -> Result<(), RecvError> {
        let event = self.rx.recv()?;

        match event {
            Event::TimerTick => app.decrement_timer(),
            Event::KeyEvent(event) => {
                Self::handle_key_event(event, app);
            },
        }
        
        Ok(())
    }
}