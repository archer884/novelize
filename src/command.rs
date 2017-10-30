use std::io::{self, Write};

pub struct Command {
    title: String,
    author: String,
}

impl Command {
    pub fn from_args() -> Self {
        let matches = clap_app!(novelize =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: "Transform a chapter listing into a novel")
            (@arg title: -t --title +required +takes_value "The title of the novel")
            (@arg author: -a --author +required +takes_value "The name of the author")
        ).get_matches();

        // Panics resulting from the following unwrap calls are rendered impossible by clap.
        let title = matches.value_of("title").unwrap().to_string();
        let author = matches.value_of("author").unwrap().to_owned();

        Self { title, author }
    }

    pub fn write_headers<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write(b"% ")?;
        w.write(self.title.as_bytes())?;
        w.write(b"\n")?;

        w.write(b"& ")?;
        w.write(self.author.as_bytes())?;
        w.write(b"\n")?;

        Ok(())
    }
}
