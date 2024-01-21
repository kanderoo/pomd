use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    no_cat: Option<bool>
    // #[command(subcommand)]
    // pub command: Command
}

#[derive(Subcommand)]
pub enum Command {
    Start,
    Status,
    Stop,
    Pause
}