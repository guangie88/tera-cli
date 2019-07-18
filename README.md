# Tera CLI

[![Codefresh build status]( https://g.codefresh.io/api/badges/pipeline/guangie88/guangie88%2Ftera-cli%2Ftera-cli?branch=master&key=eyJhbGciOiJIUzI1NiJ9.NWM4MjcyMzg3Y2NkOTUzZTcxM2RiMjRl.cTJ8XB8rM4mRl2LmZBHaIVZ92MxdGgb7Mmib1jt8o4E&type=cf-1)]( https://g.codefresh.io/pipelines/tera-cli/builds?repoOwner=guangie88&repoName=tera-cli&serviceName=guangie88%2Ftera-cli&filter=trigger:build~Build;branch:master;pipeline:5caf53ed82cf987fd3e392b9~tera-cli)
[![Crates.io](https://img.shields.io/crates/v/tera-cli.svg)](https://crates.io/crates/tera-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Tera CLI for one-off template interpolation from context file / env vars.

The following context formats are supported:

- JSON context file (`--json .` defaults to `.tera.json`)
- TOML context file (`--toml .` defaults to `.tera.toml`)
- YAML context file (`--yaml .` defaults to `.tera.yaml`)
- Environment variables

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## Simple Examples

### TOML

Template `template.tmpl`

```jinja
{% if hello %}{{ msg }}{% endif %}
```

TOML `.tera.toml`

```toml
hello = true
msg = "Hello World!"
```

STDOUT

```bash
> tera -f template.tmpl --toml .
Hello World!

> cat template.tmpl | tera --toml .
Hello World!

> tera -s "$(cat template.tmpl)" --toml .
Hello World!
```

### Environment Variables

Template `template.tmpl`

```jinja
{% if MSG %}{{ MSG }}{% endif %}
```

STDOUT

```bash
> MSG="Hello World!" tera -f template.tmpl --env
Hello World!
```

By default, setting `--toml .` looks for `.tera.toml` context file in current
working directory. Similarly, setting `--json .` looks for `.tera.json` in
current working directory. Use `--toml file_to_toml` to change the path to the
context file.

Also, the parsed value from any of the context type (i.e. TOML / JSON / env
vars) is stored into `c` as root key in the Tera context. If you wish to change
the root key to say `conf`, use `-r conf` to override.

For more details, run

```bash
tera --help
```

## Acknowledgement

Thanks to original Tera author, whose GitHub repository is at:
<https://github.com/Keats/tera>.
