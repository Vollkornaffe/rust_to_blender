use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use byteorder::{NativeEndian, WriteBytesExt};
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

    let test = 42.69;

    let mut wrt = Vec::new();
    wrt.write_f32::<NativeEndian>(test)?;

    fifo.write(&wrt)?;
    fifo.flush()?;

    Ok(())
}
