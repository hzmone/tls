use clap::Parser;
use std::io::{Result, ErrorKind, Error};

mod init;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    init: bool,
    #[arg(short, long)]
    add: Option<String>,
}

fn handle_args(args: Args) -> Result<()> {
    if args.init {
        return init::init();
    } else {
        Err(Error::new(ErrorKind::Other, "clap provided an unhandled argument"))
    }
}

fn main() {
    let args = Args::parse();
    match handle_args(args) {
        Ok(_) => return,
        Err(e) => eprintln!("unknown error occured: {}", e)
    }
}