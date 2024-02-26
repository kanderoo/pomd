use std::{io, process::exit, sync::mpsc, time::Duration};

use app::App;
use clap::Parser;
use config::Config;
use key_listener::KeyListener;
use log::Logger;
use tui::Tui;

mod args;
mod timer;
mod event;
mod tui;
mod config;
mod key_listener;
mod app;
mod notification;
mod log;

fn main() {
    let args = args::Args::parse();
    let config = Config::from_args(args);
    let logger = match Logger::new(&config.log_filepath) {
        Ok(logger) => logger,
        Err(e) => {
            eprintln!("Cannot open log file! {}", e);
            exit(-1);
        }
    };
    let (tx, rx) = mpsc::channel();

    let mut app = App::new(config, logger);
    let mut tui = match Tui::new(io::stdin(), io::stdout()) {
        Ok(tui) => tui,
        Err(e) => {
            eprintln!("Cannot enter raw mode! {}", e);
            exit(-1);
        }
    };
    
    let eh = event::EventHandler::new(rx);
    let timer = timer::Timer::new(tx.clone(), Duration::from_secs(1));
    timer.start();

    let key_listener = KeyListener::new(tx.clone());
    key_listener.start();

    loop {
        match tui.display(&mut app) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Cannot write to stdout! {}", e);
                exit(-1);
            }
        }

        match eh.handle_event(&mut app) {
            Ok(_) => (),
            Err(e) => eprintln!("Error receiving event! {}", e)
        }
        
        if app.should_quit() {
            break;
        }
    }
    
    match tui.cleanup() {
        Ok(_) => (),
        Err(e) => eprintln!("Could not exit raw mode! {}", e)
    }
}
