#[macro_use]
extern crate lazy_static;
extern crate regex;

use colored::*;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn read_log(file_path: &str, unparsed_filters: Vec<&str>, number_of_lines_option: Option<&str>) {
  let file = File::open(file_path).expect("File should exist");
  let number_of_lines = number_of_lines_option.map(|n| {
    n.parse::<i32>().expect("'lines' options should be a valid number.")
  });
  let filters = parse_filters(unparsed_filters);

  let mut count: i32 = 0;

  let stdout = io::stdout();
  let mut stdout_lock = stdout.lock();
  for maybe_line in BufReader::new(file).lines() {
    if number_of_lines.is_some() && count > number_of_lines.unwrap() {
      return;
    }
    let line = maybe_line.expect("Line should exist");
    match serde_json::from_str::<Value>(line.as_str()) {
      Err(_e) => write!(stdout_lock, "[NOT JSON] {}\n", line.red().bold()).unwrap_or(()),
      Ok(v) =>
        if passes_filters(&filters, &v) {
          count += 1;
          write!(stdout_lock, "{}\n", format_message(v)).unwrap_or(())
        },
    }
  }
}

#[derive(Debug)]
enum FilterKind {
  Equals,
  Contains,
}

#[derive(Debug)]
struct Filter {
  key: String,
  kind: FilterKind,
  value: String,
}

impl Filter {
  fn passes(&self, value: &str) -> bool {
    match self.kind {
      FilterKind::Equals => value == self.value,
      FilterKind::Contains => value.contains(self.value.as_str())
    }
  }
}

fn parse_filters(unparsed_filters: Vec<&str>) -> Vec<Filter> {
  lazy_static! {
        static ref CONTAINS_REGEX: Regex = Regex::new(r"^([^=]+)=\+([^=]+)$").unwrap();
        static ref EQUALS_REGEX: Regex = Regex::new(r"^([^=]+)=([^=]+)").unwrap();
    }

  unparsed_filters.iter().map(|text| {
    if CONTAINS_REGEX.is_match(text) {
      let caps = CONTAINS_REGEX.captures(text).unwrap();
      Filter {
        kind: FilterKind::Contains,
        key: String::from(caps.get(1).unwrap().as_str()),
        value: String::from(caps.get(2).unwrap().as_str()),
      }
    } else if EQUALS_REGEX.is_match(text) {
      let caps = EQUALS_REGEX.captures(text).unwrap();
      Filter {
        kind: FilterKind::Equals,
        key: String::from(caps.get(1).unwrap().as_str()),
        value: String::from(caps.get(2).unwrap().as_str()),
      }
    } else {
      println!("Can't parse filter: {}", text);
      panic!("Error.");
    }
  }).collect()
}

fn passes_filters(filters: &Vec<Filter>, entry: &Value) -> bool {
  filters.iter().all(|f| {
    let value = entry.get(&f.key);
    value.is_some() && f.passes(value.unwrap().as_str().unwrap())
  })
}

fn format_message(entry: Value) -> String {
  let message = entry["message"].as_str().unwrap(); // TODO handle "None"
  let maybe_level = entry["level"].as_str();

  let meta = format!(
    "[{}] [{}] [{}] [{}] [{}]",
    entry["timestamp"].as_str().unwrap(), // TODO handle "None"
    entry["level"].as_str().unwrap(), // TODO handle "None"
    entry["app"].as_str().unwrap(), // TODO handle "None"
    entry["module"].as_str().unwrap(), // TODO handle "None"
    entry["pid"].as_str().unwrap() // TODO handle "None"
  );

  format!("{} {}",
          colored_with_level(maybe_level, meta.as_str()),
          colored_with_level(maybe_level, message).bold()
  )
}

fn colored_with_level(maybe_level: Option<&str>, text: &str) -> ColoredString {
  match maybe_level {
    None => text.normal(),
    Some(level) => match level {
      "info" => text.normal(),
      "warn" => text.yellow(),
      "error" => text.red(),
      "debug" => text.blue(),
      _ => text.normal()
    }
  }
}
