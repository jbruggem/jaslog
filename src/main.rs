extern crate clap;

use clap::{Arg, App};
use jaslog::read_log;

fn main() {
  let options = App::new("jaslog")
    .version("0.1.3")
    .about("JSON logs reader for JSON logs")

    .arg(Arg::with_name("filters")
      .short("f")
      .long("filter")
      .help("Filter the logs. Example:  -f app=this -f module=+Drive (use '+' to search within the field, use '^' to exclude within the field)")
      .takes_value(true)
      .multiple(true))

    .arg(Arg::with_name("number_of_lines")
      .short("n")
      .long("lines")
      .help("Number of lines to read.")
      .takes_value(true))

    .arg(Arg::with_name("input_file")
      .help("Input file to read")
      .required(false)
      .index(1))

    .get_matches();

  let file_path = options
    .value_of("input_file");

  let lines = options
    .value_of("number_of_lines");

  let filters: Vec<&str> = match options.values_of("filters") {
    Some(elems) => elems.collect(),
    _ => Vec::new()
  };

  read_log(file_path, filters, lines);
}
