use clap::Parser;

use crate::config::PauseBehavior;

#[derive(Parser)]
pub struct Args {
    /// Length for the pomodoro in minutes
    #[arg(short, long)]
    pub pomodoro_length: Option<u32>,

    /// Length for the short break in minutes
    #[arg(short, long)]
    pub short_break_length: Option<u32>,

    /// Length for the long break in minutes
    #[arg(short, long)]
    pub long_break_length: Option<u32>,

    /// How many pomodoros before a longer break
    #[arg(short = 'c', long)]
    pub sequence_count: Option<u8>,

    /// Disable desktop notifications
    #[arg(long)]
    pub disable_notifications: bool,

    /// Change the pausing behavior
    #[arg(long)]
    pub pause_behavior: Option<PauseBehavior>,

    /// Don't display the keyboard shortcuts line
    #[arg(long)]
    pub no_help: bool,

    /// Enable task logging functionality
    #[arg(long)]
    pub log: bool
}