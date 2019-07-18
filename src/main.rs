use serde_json;
use serde_yaml;
use snafu::{Backtrace, OptionExt, ResultExt, Snafu};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};
use structopt::{clap::ArgGroup, StructOpt};
use tera::{Context, Tera};
use toml;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not read from file \"{}\": {}", path.display(), source))]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Vars file does not contain a map value"))]
    InvalidValueType,

    #[snafu(display("JSON vars parsing error: {}", source))]
    JsonParsing {
        source: serde_json::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not read from stdin: {}", source))]
    StdinRead {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Tera application error: {}", source))]
    TeraApply {
        source: tera::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("TOML vars parsing error: {}", source))]
    TomlParsing {
        source: toml::de::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("One of the YAML keys is not a string:\n{:#?}", key,))]
    YamlInvalidKey { key: serde_yaml::Value },

    #[snafu(display("YAML vars parsing error: {}", source))]
    YamlParsing {
        source: serde_yaml::Error,
        backtrace: Backtrace,
    },
}

type CliResult<T, E = Error> = std::result::Result<T, E>;

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

    /// HTML auto-escape rendered content
    #[structopt(long = "escape")]
    autoescape: bool,
}

fn read_template(conf: &Args) -> CliResult<String> {
    if let Some(ref path) = conf.input.file {
        Ok(fs::read_to_string(&path).context(FileRead { path })?)
    } else if let Some(ref content) = conf.input.string {
        Ok(content.clone())
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).context(StdinRead)?;
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
        let value = fs::read_to_string(&path)
            .context(FileRead { path })?
            .parse::<toml::Value>()
            .context(TomlParsing)?;
        let table = value.as_table().context(InvalidValueType)?;

        let mut context = Context::new();
        for (k, v) in table.iter() {
            context.insert(k, v);
        }
        Ok(context)
    } else if let Some(ref path) = conf.context.json {
        // JSON
        let path = get_config_path(path, ".json");
        let value = fs::read_to_string(&path)
            .context(FileRead { path })?
            .parse::<serde_json::Value>()
            .context(JsonParsing)?;
        let object = value.as_object().context(InvalidValueType)?;

        let mut context = Context::new();
        for (k, v) in object.iter() {
            context.insert(k, v);
        }
        Ok(context)
    } else if let Some(ref path) = conf.context.yaml {
        // YAML
        let path = get_config_path(path, ".yml");
        let value: serde_yaml::Value = serde_yaml::from_str(
            &fs::read_to_string(&path).context(FileRead { path })?,
        )
        .context(YamlParsing)?;

        // YAML specs for mapping allows for keys to be YAML value
        // so have to individually check for the root level keys to be strings
        let mapping = value.as_mapping().context(InvalidValueType)?;

        let mut context = Context::new();
        for (k, v) in mapping.iter() {
            let k = k.as_str().context(YamlInvalidKey { key: k.clone() })?;
            context.insert(k, v);
        }
        Ok(context)
    } else if conf.context.env {
        let env_vars = env::vars().collect::<HashMap<String, String>>();
        let mut context = Context::new();
        for (k, v) in env_vars.iter() {
            context.insert(k, v);
        }
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
    let rendered = Tera::one_off(&template, &context, conf.autoescape)
        .context(TeraApply)?;

    print!("{}", rendered);
    Ok(())
}
