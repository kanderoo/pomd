use std::{io, sync::mpsc, time::Duration};

use app::App;
use clap::Parser;
use config::Config;
use key_listener::KeyListener;
use tui::Tui;

mod cli;
mod timer;
mod event;
mod tui;
mod config;
mod key_listener;
mod app;

fn main() {
    let args = cli::Args::parse();
    let config = Config::from_args(args);

    let mut app = App::new(config);
    let mut tui = Tui::new(io::stdout());

    let (tx, rx) = mpsc::channel();
    
    let eh = event::EventHandler::new(rx);
    let timer = timer::Timer::new(tx.clone(), Duration::from_secs(1));
    timer.start();

    let key_listener = KeyListener::new(tx.clone());
    key_listener.start();

    loop {
        tui.display_tui(&app);

        match eh.handle_event(&mut app) {
            Ok(_) => (),
            Err(e) => eprintln!("Error receiving event! {}", e)
        }
        
        if app.should_quit() {
            break;
        }
    }
    
    tui.cleanup();
}
