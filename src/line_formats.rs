use colored::*;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use crate::format::colored_with_level;

pub trait FormatLogLine {
  fn format(&self) -> ColoredString;
}

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
    colored_with_level(&self.level, &format!("{} {}", &self.format_meta(), &self.message.bold()))
  }
}

impl ElixirLogLine {
  fn format_meta(&self) -> String {
    format!(
      "[{}] [{}] [{}] [{}] [{}]",
      self.timestamp,
      self.level,
      self.app,
      self.module,
      self.pid
    )
  }

  pub fn from(entry: &Value) -> Option<ColoredString> {
    match ElixirLogLine::deserialize(entry) {
      Err(_) => None,
      Ok(line) => Some(line.format())
    }
  }
}

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
    colored_with_level(&self.level, &format!("{} {}", &self.format_meta(), &self.message.bold()))
  }
}

impl LogstashJavaLogLine {
  fn format_meta(&self) -> String {
    format!(
      "[{}] [{}] [{}] [{}]",
      self.timestamp,
      self.level,
      self.logger_name,
      self.thread_name
    )
  }

  pub fn from(entry: &Value) -> Option<ColoredString> {
    match LogstashJavaLogLine::deserialize(entry) {
      Err(_) => None,
      Ok(line) => Some(line.format())
    }
  }
}

