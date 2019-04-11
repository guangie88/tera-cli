#[macro_use]
extern crate structopt;

use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::PathBuf,
};
use structopt::StructOpt;
use tera::{Context, Tera};

type DynError = Box<dyn Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "mode", about = "Tera CLI mode type")]
enum Mode {
    #[structopt(name = "file", about = "Read from file")]
    File {
        /// File path to read template content from
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },

    #[structopt(name = "str", about = "Read directly from argument / STDIN")]
    Str {
        /// Template content to parse. Reads from STDIN if left empty.
        #[structopt()]
        content: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "args", about = "Tera CLI arguments")]
struct Args {
    #[structopt(subcommand)]
    mode: Mode,
}

fn main() -> Result<(), DynError> {
    let conf = Args::from_args();

    let template = match conf.mode {
        Mode::File { path } => fs::read_to_string(&path)?,
        Mode::Str { content } => {
            if let Some(content) = content {
                content
            } else {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            }
        }
    };

    let context = Context::new();
    let rendered = Tera::one_off(&template, &context, true)?;
    print!("{}", rendered);

    Ok(())
}
