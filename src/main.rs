use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    blender_input_fifo: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("opening");
    let mut fifo = OpenOptions::new()
        .write(true)
        .open(args.blender_input_fifo)?;

    println!("writing");
    fifo.write(b"Hello from rust")?;
    fifo.flush()?;

    Ok(())
}
