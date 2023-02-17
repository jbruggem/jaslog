#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;

// Used for tests
#[cfg_attr(test, macro_use)]
extern crate serde_json;

use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub mod filter;
pub mod format;
pub mod line_formats;

use filter::*;
use format::*;

pub fn read_log(
  maybe_file_path: Option<&String>,
  unparsed_filters: Vec<&String>,
  number_of_lines: Option<&u64>,
) {
  let stdin = io::stdin();
  let reader: Box<dyn BufRead> = match maybe_file_path {
    Some(file_path) => Box::new(BufReader::new(
      File::open(file_path).expect("File should exist"),
    )),
    None => Box::new(stdin.lock()),
  };

  let filters = parse_filters(unparsed_filters);

  let mut count: u64 = 0;

  let stdout = io::stdout();
  let mut stdout_lock = stdout.lock();
  let mut formatter = Formatter::new();
  for maybe_line in reader.lines() {
    if number_of_lines.is_some() && count >= *number_of_lines.unwrap() {
      return;
    }
    let line = maybe_line.expect("Line should exist");
    let output = match serde_json::from_str::<Value>(line.as_str()) {
      Err(_e) => Some(formatter.format_not_json(&line)),
      Ok(v) => {
        if v.is_object() {
          if passes_filters(&filters, &v) {
            Some(formatter.format_message(v))
          } else {
            None
          }
        } else {
          Some(formatter.format_not_json(&line))
        }
      }
    };

    if output.is_some() {
      writeln!(stdout_lock, "{}", output.unwrap()).unwrap_or(());
      count += 1;
    }
  }
}
