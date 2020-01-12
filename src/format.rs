use colored::*;
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ElixirLogLine {
  app: String,
  level: String,
  message: String,
  module: String,
  pid: String,
  timestamp: String,
}

pub fn format_not_json(line: &str) -> String {
  format!("{} {}", "[NOT JSON]".red(), line.bold())
}

pub fn format_message(entry: Value) -> String {
  match ElixirLogLine::deserialize(&entry) {
    Err(_) => format_raw(&entry),
    Ok(fields) => format(&fields)
  }
}

pub fn format_raw(entry: &Value) -> String {
  match entry {
    Value::Object(map) => {
      let text = map.iter().map(|(_key, value)| {
        format!("[{}]", value.to_string().bold())
      }).collect::<Vec<String>>().join(" ");

      format!("{}", colored_with_maybe_level(entry.get("level").unwrap_or(&Value::Null).as_str(), &text))
    }
    _ => panic!("Unsupported parsed json")
  }
}

pub fn format(line: &ElixirLogLine) -> String {
  format!("{} {}",
          colored_with_level(&line.level, &format_meta(&line)),
          colored_with_level(&line.level, &line.message).bold()
  )
}

fn format_meta(line: &ElixirLogLine) -> String {
  format!(
    "[{}] [{}] [{}] [{}] [{}]",
    line.timestamp,
    line.level,
    line.app,
    line.module,
    line.pid
  )
}

fn colored_with_maybe_level(maybe_level: Option<&str>, text: &str) -> ColoredString {
  match maybe_level {
    None => text.normal(),
    Some(level) => colored_with_level(level, text)
  }
}

fn colored_with_level(level: &str, text: &str) -> ColoredString {
  match level {
    "info" => text.normal(),
    "warn" => text.yellow(),
    "error" => text.red(),
    "debug" => text.blue(),
    _ => text.normal()
  }
}
