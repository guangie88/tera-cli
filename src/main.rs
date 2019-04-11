use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::PathBuf,
};
use clap::arg_enum;
use structopt::StructOpt;
use tera::{Context, Tera};
use toml::Value;

type DynError = Box<dyn Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "input", about = "Tera CLI input type")]
enum Input {
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

arg_enum! {
    #[derive(Debug)]
    enum ContextType {
        Toml,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "args", about = "Tera CLI arguments")]
struct Args {
    #[structopt(subcommand)]
    input: Input,

    #[structopt(raw(possible_values = "&ContextType::variants()", case_insensitive = "true"))]
    context: ContextType,

    #[structopt(parse(from_os_str))]
    context_path: Option<PathBuf>,
}

fn main() -> Result<(), DynError> {
    let conf = Args::from_args();

    let template = match conf.input {
        Input::File { path } => fs::read_to_string(&path)?,
        Input::Str { content } => {
            if let Some(content) = content {
                content
            } else {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            }
        }
    };

    let context = match conf.context {
        ContextType::Toml => {
            let context_path = if let Some(context_path) = conf.context_path {
                context_path
            } else {
                PathBuf::from(".tera.toml")
            };

            let content = fs::read_to_string(&context_path)?;
            let value = content.parse::<Value>()?;

            let mut context = Context::new();
            context.insert("c", &value);
            context
        }
    };

    let rendered = Tera::one_off(&template, &context, true)?;
    print!("{}", rendered);

    Ok(())
}
