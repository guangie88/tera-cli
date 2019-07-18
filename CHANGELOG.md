# CHANGELOG

## Unreleased

- Drops root key support entirely. Only allow variable file to contain
  `map[string, value]` at root. The keys of the map now forms the root keys.

  That is, if env var `MSG` has the value `"hello"`, previously the template
  string to use was `{{ c.MSG }}`. Now the template string to use should be
  `{{ MSG }}`.

## `v0.1.1`

- Add YAML support to CLI ([#3](https://github.com/guangie88/tera-cli/pull/3)).

## `v0.1.0`

- Initial implementation with JSON, TOML and env var support.
- Reads all possible values into a default root key `c`.
