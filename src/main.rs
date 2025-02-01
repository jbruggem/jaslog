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
      .action(clap::ArgAction::Append))

    .arg(Arg::new("number_of_lines")
      .short('n')
      .long("lines")
      .help("Number of lines to read.")
      .num_args(1)
      .value_parser(clap::value_parser!(u64))
      .action(clap::ArgAction::Set))


    .arg(Arg::new("input_file")
      .help("Input file to read")
      .required(false)
      .index(1)
      .action(clap::ArgAction::Set))

    .get_matches();

  let file_path = options.get_one::<String>("input_file");

  let lines = options.get_one::<u64>("number_of_lines");

  let filters: Vec<&str> = options
    .get_many::<String>("filters")
    .map(|filters| filters.clone().map(|s| s.as_str()).collect())
    .unwrap_or_default();

  read_log(file_path, filters, lines);
}
