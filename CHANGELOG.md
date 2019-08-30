# CHANGELOG

## `v0.2.1`

- Fix and return non-zero exit code during error
  ([#6](https://github.com/guangie88/tera-cli/pull/6)).

## `v0.2.0`

- Drop root key support entirely
  ([#4](https://github.com/guangie88/tera-cli/pull/4)).
  Only allow variable file to contain `map[string, value]` at root. The keys of
  the map now forms the root keys.

  That is, if env var `MSG` has the value `"hello"`, previously the template
  string to use was `{{ c.MSG }}`. Now the template string to use should be
  `{{ MSG }}`.

## `v0.1.1`

- Add YAML support to CLI ([#3](https://github.com/guangie88/tera-cli/pull/3)).

## `v0.1.0`

- Initial implementation with JSON, TOML and env var support.
- Read all possible values into a default root key `c`.
