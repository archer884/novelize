use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Stdin, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Command {
    #[structopt(short = "t", long = "title", help = "The title of this work")]
    title: String,
    #[structopt(short = "a", long = "author", help = "The author of this work")]
    author: String,
}

impl Command {
    pub fn write_headers(&self, w: &mut impl Write) -> io::Result<()> {
        write!(w, "% {}\n% {}\n\n", self.title, self.author)
    }
}

fn main() -> io::Result<()> {
    run(Command::from_args())
}

fn run(command: Command) -> io::Result<()> {
    let files = read_listing(&io::stdin())?;
    let out = io::stdout();
    let out = out.lock();
    let mut out = BufWriter::new(out);

    command.write_headers(&mut out)?;
    for file in files {
        io::copy(&mut open(&file)?, &mut out)?;
        out.write_all(b"\n")?;
    }

    out.flush()
}

fn open(s: &str) -> io::Result<BufReader<File>> {
    File::open(s.trim()).map(BufReader::new)
}

fn read_listing(stdin: &Stdin) -> io::Result<Vec<String>> {
    stdin.lock().lines().collect()
}
