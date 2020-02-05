#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;

// Used for tests
#[macro_use]
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
  maybe_file_path: Option<&str>,
  unparsed_filters: Vec<&str>,
  number_of_lines_option: Option<&str>,
) {
  let stdin = io::stdin();
  let reader: Box<dyn BufRead> = match maybe_file_path {
    Some(file_path) => Box::new(BufReader::new(File::open(file_path).expect("File should exist"))),
    None => Box::new(stdin.lock()),
  };

  let number_of_lines = number_of_lines_option.map(|n| {
    n.parse::<i32>()
      .expect("'lines' options should be a valid number.")
  });
  let filters = parse_filters(unparsed_filters);

  let mut count: i32 = 0;

  let stdout = io::stdout();
  let mut stdout_lock = stdout.lock();
  for maybe_line in reader.lines() {
    if number_of_lines.is_some() && count > number_of_lines.unwrap() {
      return;
    }
    let line = maybe_line.expect("Line should exist");
    match serde_json::from_str::<Value>(line.as_str()) {
      Err(_e) => write!(stdout_lock, "{}\n", format_not_json(&line)).unwrap_or(()),
      Ok(v) => {
        if passes_filters(&filters, &v) {
          count += 1;
          write!(stdout_lock, "{}\n", format_message(v)).unwrap_or(())
        }
      }
    }
  }
}
