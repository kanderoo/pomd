use std::{io, process::exit, sync::mpsc, time::Duration};

use clap::Parser;
use config::Config;
use crossterm::event::{KeyCode, ModifierKeyCode};
use key_listener::KeyListener;

use crate::event::Event;

mod cli;
mod timer;
mod event;
mod tui;
mod config;
mod key_listener;

struct State {
    remaining_time: Duration,
    pom_phase: PomodoroPhase,
    pom_count: u8,
    paused: bool
}

enum PomodoroPhase {
    Work,
    Break,
    LongBreak
}

fn main() {
    let _args = cli::Args::parse();
    let config = Config::default();

    let mut state = State {
        remaining_time: config.work_duration,
        pom_count: 0,
        pom_phase: PomodoroPhase::Work,
        paused: false
    };
    
    
    let mut tui = tui::Tui::new(io::stdout());

    let (tx, rx) = mpsc::channel();
    
    let eh = event::EventHandler::new(rx);
    let timer = timer::Timer::new(tx.clone(), Duration::from_secs(1));
    timer.start();

    let key_listener = KeyListener::new(tx.clone());
    key_listener.start();

    loop {
        tui.display_tui(&state);

        let event = eh.poll_event();

        match event {
            Event::TimerTick => {
                if !state.paused {
                    state.remaining_time -= Duration::from_secs(1);
                    if state.remaining_time.is_zero() {
                        next_phase(&mut state, &config);
                    }
                }
            },
            Event::KeyEvent(event) => {
                if perform_key_action(event.code, &mut state) {
                    break;
                } 
            }
        }
    }
}

fn perform_key_action(code: KeyCode, state: &mut State) -> bool {
    match code {
        KeyCode::Char('q') => {
            return true
        },
        KeyCode::Char(' ') => {
            state.paused = !state.paused;
        },
        _ => ()
    }
    false
}

fn next_phase(state: &mut State, config: &Config) {
    match state.pom_phase {
        PomodoroPhase::Work => {
            if state.pom_count == 4 {
                state.pom_phase = PomodoroPhase::LongBreak;
                state.remaining_time = config.long_break_duration;
            } else {
                state.pom_phase = PomodoroPhase::Break;
                state.remaining_time = config.short_break_duration;
            }
            state.pom_count += 1;
        },
        PomodoroPhase::Break | PomodoroPhase::LongBreak => {
            state.pom_phase = PomodoroPhase::Work;
            state.remaining_time = config.work_duration;
        }
    }
    state.paused = true;
}