use std::io::{self, Write};

#[derive(Debug, StructOpt)]
pub struct Command {
    #[structopt(short = "t", long = "title", help = "The title of this work")]
    title: String,

    #[structopt(short = "a", long = "author", help = "The author of this work")]
    author: String,
}

impl Command {
    pub fn from_args() -> Self {
        use structopt::StructOpt;
        <Self as StructOpt>::from_args()
    }

    pub fn write_headers<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"% ")?;
        w.write_all(self.title.as_bytes())?;
        w.write_all(b"\n")?;

        w.write_all(b"% ")?;
        w.write_all(self.author.as_bytes())?;
        w.write_all(b"\n\n")?;

        Ok(())
    }
}
