#[macro_use] extern crate clap;

mod command;

use command::Command;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result, Write};

fn main() {
    use std::error::Error;

    if let Err(e) = run(Command::from_args) {
        println!("{}", e.description());
        std::process::exit(1);
    }
}

fn run<F: Fn() -> Command>(command_provider: F) -> Result<()> {
    let command = command_provider();
    let files = read_listing()?;

    let out = io::stdout();
    let mut out = out.lock();

    command.write_headers(&mut out)?;

    for file in files {
        io::copy(&mut open(file)?, &mut out)?;
        out.write(b"\n")?;
    }

    out.flush()
}

fn open(s: String) -> io::Result<BufReader<File>> {
    File::open(s.trim()).map(|file| BufReader::new(file))
}

fn read_listing() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let result = stdin.lines().collect();
    result
}
