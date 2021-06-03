use crate::format::colored_with_level;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait FormatLogLine {
  fn format(&self) -> ColoredString;
}

pub trait ToColoredString {
  fn to_colored_string(entry: &Value) -> Option<ColoredString>;
}

//////////////////////////////////
/// ElixirLogLine
//////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct ElixirLogLine {
  app: String,
  level: String,
  message: String,
  module: String,
  pid: String,
  timestamp: String,
}

impl FormatLogLine for ElixirLogLine {
  fn format(&self) -> ColoredString {
    colored_with_level(
      &self.level,
      &format!("{} {}", &self.format_meta(), &self.message.bold()),
    )
  }
}

impl ElixirLogLine {
  fn format_meta(&self) -> String {
    format!(
      "[{}] [{}] [{}] [{}] [{}]",
      self.timestamp, self.level, self.app, self.module, self.pid
    )
  }
}

impl ToColoredString for ElixirLogLine {
  fn to_colored_string(entry: &Value) -> Option<ColoredString> {
    match ElixirLogLine::deserialize(entry) {
      Err(_) => None,
      Ok(line) => Some(line.format()),
    }
  }
}

//////////////////////////////////
/// LogstashJavaLogLine
//////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct LogstashJavaLogLine {
  level: String,
  message: String,
  logger_name: String,
  thread_name: String,
  #[serde(alias = "@timestamp")]
  timestamp: String,
  // --- unused
  // level_value: i16,
  // @version: 1,

  // --- optional
  // stack_trace: String
  // tags: list of tags
}

impl FormatLogLine for LogstashJavaLogLine {
  fn format(&self) -> ColoredString {
    colored_with_level(
      &self.level,
      &format!("{} {}", &self.format_meta(), &self.message.bold()),
    )
  }
}

impl ToColoredString for LogstashJavaLogLine {
  fn to_colored_string(entry: &Value) -> Option<ColoredString> {
    match LogstashJavaLogLine::deserialize(entry) {
      Err(_) => None,
      Ok(line) => Some(line.format()),
    }
  }
}

impl LogstashJavaLogLine {
  fn format_meta(&self) -> String {
    format!(
      "[{}] [{}] [{}] [{}]",
      self.timestamp, self.level, self.logger_name, self.thread_name
    )
  }
}
