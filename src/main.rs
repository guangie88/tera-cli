// #[macro_use]
// extern crate serde_derive;

// use clap::arg_enum;
use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::PathBuf,
};
use structopt::{clap::ArgGroup, StructOpt};
use tera::{Context, Tera};
use toml::Value;

type DynError = Box<dyn Error>;
type CliResult<T> = Result<T, DynError>;

fn input_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("input").required(true)
}

fn format_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("format").required(true)
}

#[derive(StructOpt, Debug)]
#[structopt(raw(group = "input_arg_group()"))]
struct Input {
    /// File path to read template content from
    #[structopt(
        name = "file",
        long,
        short,
        group = "input",
        parse(from_os_str)
    )]
    file: Option<PathBuf>,

    /// Read directly from argument / STDIN
    #[structopt(name = "str", long, short, group = "input")]
    string: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(raw(group = "format_arg_group()"))]
struct ContextFormat {
    #[structopt(name = "toml", long, group = "format", parse(from_os_str))]
    toml: Option<PathBuf>,

    #[structopt(name = "toml", long, group = "format")]
    env: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "args", about = "Tera CLI arguments")]
struct Args {
    #[structopt(flatten)]
    input: Input,

    #[structopt(flatten)]
    context: ContextFormat,

    /// Root key to embed the context configuration into.
    #[structopt(short = "r", long = "root", default_value = "c")]
    root_key: String,
}

fn read_template(conf: &Args) -> CliResult<String> {
    if let Some(ref path) = conf.input.file {
        Ok(fs::read_to_string(&path)?)
    } else if let Some(ref content) = conf.input.string {
        Ok(content.clone())
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

fn read_context(conf: &Args) -> CliResult<Context> {
    if let Some(ref path) = conf.context.toml {
        let value = fs::read_to_string(path)?.parse::<Value>()?;
        let mut context = Context::new();
        context.insert(&conf.root_key, &value);
        Ok(context)
    } else {
        // else if conf.context.env
        Ok(Context::new())
    }
}

fn main() -> CliResult<()> {
    let conf = Args::from_args();

    let template = read_template(&conf)?;
    let context = read_context(&conf)?;
    let rendered = Tera::one_off(&template, &context, true)?;
    print!("{}", rendered);

    Ok(())
}
