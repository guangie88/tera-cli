# Tera CLI

[![CI Status](https://img.shields.io/github/workflow/status/guangie88/tera-cli/ci/master?label=ci&logo=github&style=for-the-badge)](https://github.com/guangie88/tera-cli/actions)
[![Crates.io](https://img.shields.io/crates/v/tera-cli?style=for-the-badge)](https://crates.io/crates/tera-cli)
[![License: MIT](https://img.shields.io/github/license/guangie88/tera-cli?style=for-the-badge)](https://opensource.org/licenses/MIT)

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

For more details, run

```bash
tera --help
```

## Acknowledgement

Thanks to original Tera author, whose GitHub repository is at:
<https://github.com/Keats/tera>.

Also thanks to `BurntSushi` and his `ripgrep` repository showcase of GitHub
Actions for cross-compilation, which has been copied and adapted heavily into
this repository. The original files are here:
<https://github.com/BurntSushi/ripgrep/tree/8905d54a9f25f4c1e4e3ca8331f517473e174d87/.github/workflows>.
