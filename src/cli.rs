use clap::Parser;

#[derive(Parser)]
pub struct Args {
    no_cat: Option<bool>
}