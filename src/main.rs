use std::{io, sync::mpsc, time::Duration};

use clap::Parser;
use config::Config;

use crate::event::Event;

mod cli;
mod timer;
mod event;
mod tui;
mod config;

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

    loop {
        tui.display_tui(&state);

        let event = eh.poll_event();

        match event {
            Event::TimerTick => {
                state.remaining_time -= Duration::from_secs(1);
                if state.remaining_time.is_zero() {
                    next_phase(&mut state, &config);
                }
            }
        }
    }
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