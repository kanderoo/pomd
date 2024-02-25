use std::{io, sync::mpsc, thread::{self, sleep, JoinHandle}, time::Duration};

use clap::Parser;
use timer::Timer;

use crate::event::Event;

mod cli;
mod timer;
mod event;
mod tui;

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
    let args = cli::Args::parse();

    let mut state = State {
        remaining_time: Duration::from_secs(15),
        pom_count: 0,
        pom_phase: PomodoroPhase::Work,
        paused: false
    };
    
    let mut tui = tui::Tui::new(io::stdout());

    let (tx, rx) = mpsc::channel();
    
    let eh = event::EventHandler::new(rx);
    let timer = timer::Timer::new(tx.clone(), Duration::from_secs(1));
    timer.start();

    loop {
        tui.display_tui(&state);

        let event = eh.poll_event();

        match event {
            Event::TimerTick => {
                state.remaining_time -= Duration::from_secs(1);
                if state.remaining_time.is_zero() {
                    next_phase(&mut state);
                }
            }
        }
    }
}


fn next_phase(state: &mut State) {
    match state.pom_phase {
        PomodoroPhase::Work => {
            if state.pom_count == 4 {
                state.pom_phase = PomodoroPhase::LongBreak;
                state.remaining_time = Duration::from_secs(15);
            } else {
                state.pom_phase = PomodoroPhase::Break;
                state.remaining_time = Duration::from_secs(5);
            }
            state.pom_count += 1;
        },
        PomodoroPhase::Break | PomodoroPhase::LongBreak => state.pom_phase = PomodoroPhase::Work,
    }
}