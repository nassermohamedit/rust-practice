use flate2::{Compression, GzBuilder};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::exit;

/// The basic-est compression tool ever!

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: env::Args = env::args();
    if args.len() < 3 {
        eprintln!("Usage: [file_path] [compressed_file_path]");
        exit(1);
    }
    args.next();
    let file_path = args.next().unwrap();
    let compressed_file_path = args.next().unwrap();
    println!("arg1 = {file_path}, arg2 = {compressed_file_path}");

    let mut input = BufReader::new(File::open(&file_path)?);
    let output = fs::File::create(compressed_file_path)?;
    let mut gz = GzBuilder::new()
        .filename(file_path)
        .write(output, Compression::default());
    let mut buf = [0u8; 100];

    loop {
        let n = input.read(&mut buf)?;
        if n <= 0 {
            break;
        }
        println!("{n}");
        gz.write_all(&buf[..n]).expect("Failed compression. Oops!");
    }
    Ok(())
}
