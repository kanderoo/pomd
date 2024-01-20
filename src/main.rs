use clap::Parser;
use cli::Command;

mod cli;
fn main() {
    let args = cli::Args::parse();
    
    match args.command {
        Command::Start => start_timer(),
        _ => todo!("make other commands work")
    }
}


fn start_timer() {

}