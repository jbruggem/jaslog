extern crate clap;

use clap::{Arg, Command};
use jaslog::read_log;

fn main() {
  const VERSION: &str = env!("CARGO_PKG_VERSION");

  let options = Command::new("jaslog")
    .version(VERSION)
    .about("JSON logs reader for JSON logs")

    .arg(Arg::new("filters")
      .short('f')
      .long("filter")
      .help("Filter the logs. Example:  -f app=this -f module=+Drive (use '+' to search within the field, use '^' to exclude within the field)")
      .takes_value(true)
      .number_of_values(1)
      .multiple_occurrences(true))

    .arg(Arg::new("number_of_lines")
      .short('n')
      .long("lines")
      .help("Number of lines to read.")
      .takes_value(true))

    .arg(Arg::new("input_file")
      .help("Input file to read")
      .required(false)
      .index(1))

    .get_matches();

  let file_path = options.value_of("input_file");

  let lines = options.value_of("number_of_lines");

  let filters: Vec<&str> = match options.values_of("filters") {
    Some(elems) => elems.collect(),
    _ => Vec::new(),
  };

  read_log(file_path, filters, lines);
}
