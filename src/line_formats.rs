use crate::format::colored_with_level;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
extern crate chrono;
use chrono::prelude::*;

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
      &format!("{} {}", &self.format_meta().dimmed(), &self.message),
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
      &format!("{} {}", &self.format_meta().dimmed(), &self.message),
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

//////////////////////////////////
/// Log4J's default JSONLayout
//////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct Log4JJsonLayoutLogLine {
  #[serde(alias = "thread")]
  thread_name: String,
  level: String,
  #[serde(alias = "loggerName")]
  logger_name: String,
  #[serde(alias = "endOfBatch")]
  end_of_batch: bool,
  #[serde(alias = "loggerFqcn")]
  logger_fqcn: String,
  message: String,

  instant: Log4JJsonLayoutLogLineInstant,
  #[serde(alias = "threadId")]
  thread_id: i32,
  #[serde(alias = "threadPriority")]
  thread_priority: i32,
  #[serde(default)]
  thrown: Log4JJsonLayoutLogLineThrown,
}

#[derive(Serialize, Deserialize, Default)]
struct Log4JJsonLayoutLogLineThrown {
  #[serde(alias = "commonElementCount")]
  common_element_count: i32,
  #[serde(alias = "localizedMessage")]
  localized_message: String,
  message: String,
  name: String,
}

#[derive(Serialize, Deserialize)]
struct Log4JJsonLayoutLogLineInstant {
  #[serde(alias = "epochSecond")]
  epoch_second: i64,
  #[serde(alias = "nanoOfSecond")]
  nano_of_second: u32,
}

impl FormatLogLine for Log4JJsonLayoutLogLine {
  fn format(&self) -> ColoredString {
    colored_with_level(
      &self.level,
      &format!(
        "{} {}{}",
        &self.format_meta().dimmed(),
        &self.message,
        &self.format_stacktrace()
      ),
    )
  }
}

impl ToColoredString for Log4JJsonLayoutLogLine {
  fn to_colored_string(entry: &Value) -> Option<ColoredString> {
    match Log4JJsonLayoutLogLine::deserialize(entry) {
      Err(_) => None,
      Ok(line) => Some(line.format()),
    }
  }
}

impl Log4JJsonLayoutLogLine {
  fn format_stacktrace(&self) -> String {
    if !self.thrown.message.is_empty() {
      format!(
        "\n\t{}\n\t{}",
        self.thrown.name,
        self.thrown.message.replace("\n", "\t\n")
      )
    } else {
      "".to_string()
    }
  }

  fn format_meta(&self) -> String {
    let naive_datetime =
      NaiveDateTime::from_timestamp(self.instant.epoch_second, self.instant.nano_of_second);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    format!(
      "[{}] [{}] [{}] [{}]",
      datetime.format("%+"),
      self.level,
      self.logger_name,
      self.thread_name
    )
  }
}
