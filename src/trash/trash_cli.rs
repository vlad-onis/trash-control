use clap::Parser;

use std::fmt::Debug;

#[derive(Parser, Debug)]
pub struct Args {
    /// List files in the thrash bin
    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long)]
    pub empty: bool,
}

pub fn read_cli_arg() -> Args {
    Args::parse()
}
