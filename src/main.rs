use serde_json;
use serde_yaml;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};
use structopt::{clap::ArgGroup, StructOpt};
use tera::{Context, Tera};
use toml;

type DynError = Box<dyn Error>;
type CliResult<T> = Result<T, DynError>;

fn input_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("input")
}

fn format_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("format")
}

/// Leave empty to read from STDIN
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

    /// Read directly from argument
    #[structopt(name = "str", long, short, group = "input")]
    string: Option<String>,
}

/// Context format file type to read from.
#[derive(StructOpt, Debug)]
#[structopt(raw(group = "format_arg_group()"))]
struct ContextFormat {
    /// TOML file path to read context values.
    /// "." to indicate reading from default ".tera.toml"
    #[structopt(name = "toml", long, group = "format", parse(from_os_str))]
    toml: Option<PathBuf>,

    /// JSON file path to read context values.
    /// "." to indicate reading from default ".tera.json"
    #[structopt(name = "json", long, group = "format", parse(from_os_str))]
    json: Option<PathBuf>,

    /// YAML file path to read context values.
    /// "." to indicate reading from default ".tera.yml"
    #[structopt(name = "yaml", long, group = "format", parse(from_os_str))]
    yaml: Option<PathBuf>,

    /// Use env vars as the context instead
    #[structopt(name = "env", long, group = "format")]
    env: bool,
}

/// Tera CLI arguments
#[derive(StructOpt, Debug)]
#[structopt(
    name = "Tera CLI",
    about = "Tera CLI to apply template using config values / env vars"
)]
struct Args {
    #[structopt(flatten)]
    input: Input,

    #[structopt(flatten)]
    context: ContextFormat,

    /// Root key to embed the context configuration into
    #[structopt(short = "r", long = "root", default_value = "c")]
    root_key: String,

    /// HTML auto-escape rendered content
    #[structopt(long = "escape")]
    autoescape: bool,
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

fn get_config_path(path: &Path, ext: &str) -> PathBuf {
    if path.as_os_str() != "." {
        path.to_path_buf()
    } else {
        PathBuf::from(format!(".tera{}", ext))
    }
}

fn read_context(conf: &Args) -> CliResult<Context> {
    if let Some(ref path) = conf.context.toml {
        // TOML
        let path = get_config_path(path, ".toml");
        let value = fs::read_to_string(&path)?.parse::<toml::Value>()?;
        let mut context = Context::new();
        context.insert(&conf.root_key, &value);
        Ok(context)
    } else if let Some(ref path) = conf.context.json {
        // JSON
        let path = get_config_path(path, ".json");
        let value = fs::read_to_string(&path)?.parse::<serde_json::Value>()?;
        let mut context = Context::new();
        context.insert(&conf.root_key, &value);
        Ok(context)
    } else if let Some(ref path) = conf.context.yaml {
        // YAML
        let path = get_config_path(path, ".yml");
        let value: serde_yaml::Value =
            serde_yaml::from_str(&fs::read_to_string(&path)?)?;
        let mut context = Context::new();
        context.insert(&conf.root_key, &value);
        Ok(context)
    } else if conf.context.env {
        let env_vars = env::vars().collect::<HashMap<String, String>>();
        let mut context = Context::new();
        context.insert(&conf.root_key, &env_vars);
        Ok(context)
    } else {
        // Empty context, useless but still valid
        Ok(Context::new())
    }
}

fn main() -> CliResult<()> {
    let conf = Args::from_args();

    // Read context first because the template might possibly be read from STDIN
    let context = read_context(&conf)?;
    let template = read_template(&conf)?;
    let rendered = Tera::one_off(&template, &context, conf.autoescape)?;

    print!("{}", rendered);
    Ok(())
}
