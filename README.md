# Jaslog

A log reader for structured logs.

For now, it only works with these fields in the original json (typical of an Elixir app):

```json
{
  "app": "ecto_sql",
  "level": "info",
  "message": "create index etc...",
  "metadata": {},
  "module": "Elixir.Ecto.Migration.Runner",
  "pid": "#PID<0.280.0>",
  "timestamp": "2019-12-18T10:55:50.000393"
}
```

Usage:

```bash
jason 0.0.1
JSON logs reader for JSON logs

USAGE:
    jaslog [OPTIONS] <input_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <filters>...        Filter the logs. Example:  -f app=this -f module=+Drive (use '+' to search within
                                     the field)
    -n, --lines <number_of_lines>    Number of lines to read.

ARGS:
    <input_file>    Input file to read
```
