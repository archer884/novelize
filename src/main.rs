extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod command;

use command::Command;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result, Stdin, Write};

fn main() {
    use std::error::Error;

    if let Err(e) = run(Command::from_args()) {
        println!("{}", e.description());
        std::process::exit(1);
    }
}

fn run(command: Command) -> Result<()> {
    let files = read_listing(&io::stdin())?;
    let out = io::stdout();
    let mut out = out.lock();

    command.write_headers(&mut out)?;
    for file in files {
        io::copy(&mut open(&file)?, &mut out)?;
        out.write_all(b"\n")?;
    }

    out.flush()
}

fn open(s: &str) -> Result<BufReader<File>> {
    File::open(s.trim()).map(BufReader::new)
}

fn read_listing(stdin: &Stdin) -> Result<Vec<String>> {
    stdin.lock().lines().collect()
}
