use crate::format::colored_with_level;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
extern crate chrono;
use chrono::prelude::*;

pub trait FormatLogLine {
  fn format(&self) -> ColoredString;
}

pub trait ToColoredString {
  fn to_colored_string(entry: &Value) -> Option<ColoredString>;
}

fn format_mdc(mdc: &Map<String, Value>) -> String {
  if !mdc.is_empty() {
    let res = mdc
      .clone()
      .into_iter()
      .map(|(key, value)| {
        let shown_value = match value {
          Value::String(val) => val,
          other => format!("{other:?}"),
        };
        format!("{key}={}", shown_value.trim())
      })
      .collect::<Vec<String>>();
    format!("[{}]", res.join(","))
  } else {
    "".to_string()
  }
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
/// ElixirExtendedLogLine
//////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct ElixirExtendedLogLine {
  application: String,
  level: String,
  message: String,
  module: String,
  pid: String,
  timestamp: String,
}

impl FormatLogLine for ElixirExtendedLogLine {
  fn format(&self) -> ColoredString {
    colored_with_level(
      &self.level,
      &format!("{} {}", &self.format_meta().dimmed(), &self.message),
    )
  }
}

impl ElixirExtendedLogLine {
  fn format_meta(&self) -> String {
    format!(
      "[{}] [{}] [{}] [{}] [{}]",
      self.timestamp, self.level, self.application, self.module, self.pid
    )
  }
}

impl ToColoredString for ElixirExtendedLogLine {
  fn to_colored_string(entry: &Value) -> Option<ColoredString> {
    match ElixirExtendedLogLine::deserialize(entry) {
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
  // tags: list of tags (optional)
  #[serde(default)]
  mdc: Map<String, Value>,

  #[serde(default)]
  stack_trace: String,
  #[serde(default)]
  exception: LogstashLogLineException,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
struct LogstashLogLineException {
  #[serde(default, alias = "exception_message")]
  message: String,
  #[serde(default, alias = "exception_class")]
  class: String,
  #[serde(default)]
  stacktrace: String,
}

impl FormatLogLine for LogstashJavaLogLine {
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
      "[{}] [{}] [{}] [{}]{}",
      self.timestamp,
      self.level,
      self.logger_name,
      self.thread_name,
      self.format_mdc()
    )
  }
  fn format_stacktrace(&self) -> ColoredString {
    if !self.exception.message.is_empty() {
      format!(
        "\n\t{} ({})\n\t{}",
        self.exception.message,
        self.exception.class,
        self.exception.stacktrace.replace('\n', "\n\t")
      )
      .red()
    } else if !self.stack_trace.is_empty() {
      format!("\n\t{}", self.stack_trace.replace('\n', "\n\t")).red()
    } else {
      "".normal()
    }
  }

  fn format_mdc(&self) -> String {
    format_mdc(&self.mdc)
  }
}

//////////////////////////////////
/// Log4J's default JSONLayout
//////////////////////////////////

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
  #[serde(default)]
  mdc: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
struct Log4JJsonLayoutLogLineThrown {
  #[serde(alias = "commonElementCount", default)]
  common_element_count: i32,
  #[serde(alias = "localizedMessage", default)]
  localized_message: String,
  #[serde(default)]
  message: String,
  #[serde(default)]
  name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
  fn format_stacktrace(&self) -> ColoredString {
    if !self.thrown.message.is_empty() && !self.thrown.name.is_empty() {
      format!(
        "\n\t{}\n\t{}",
        self.thrown.name,
        self.thrown.message.replace('\n', "\n\t")
      )
      .red()
    } else if !self.thrown.message.is_empty() && self.thrown.name.is_empty() {
      format!("\n\t{}", self.thrown.message.replace('\n', "\n\t")).red()
    } else if !self.thrown.name.is_empty() {
      format!(" ({})", self.thrown.name).red()
    } else {
      "".normal()
    }
  }

  fn format_meta(&self) -> String {
    format!(
      "[{}] [{}] [{}] [{}]{}",
      self.format_date(),
      self.level,
      self.logger_name,
      self.thread_name,
      self.format_mdc()
    )
  }

  fn format_mdc(&self) -> String {
    format_mdc(&self.mdc)
  }

  fn format_date(&self) -> String {
    NaiveDateTime::from_timestamp_opt(self.instant.epoch_second, self.instant.nano_of_second)
      .map(|naive_datetime| DateTime::from_utc(naive_datetime, Utc))
      .map(|datetime: DateTime<Utc>| datetime.format("%+").to_string())
      .unwrap_or(self.instant.epoch_second.to_string())
  }
}

// {
//   "epochSecond": 1622724607,
//   "nanoOfSecond": 420000000
// }

// {"epochSecond":1675671221,"nanoOfSecond":782873000}

#[cfg(test)]
mod tests {

  use super::*;

  fn with_epoch_and_nano_format_date(epoch_second: i64, nano_of_second: u32) -> String {
    Log4JJsonLayoutLogLine {
      thread_name: String::new(),
      level: String::new(),
      logger_name: String::new(),
      end_of_batch: true,
      logger_fqcn: String::new(),
      message: String::new(),
      instant: Log4JJsonLayoutLogLineInstant {
        epoch_second,
        nano_of_second,
      },
      thread_id: 0,
      thread_priority: 0,
      thrown: Log4JJsonLayoutLogLineThrown::default(),
      mdc: Map::new(),
    }
    .format_date()
  }

  #[test]
  fn test_epoch_formatting_works() {
    assert_eq!(
      with_epoch_and_nano_format_date(1622724607, 420000000),
      "2021-06-03T12:50:07.420+00:00"
    );
    assert_eq!(
      with_epoch_and_nano_format_date(1675671221, 782873000),
      "2023-02-06T08:13:41.782873+00:00"
    );
    assert_eq!(
      with_epoch_and_nano_format_date(1675671481, 452180000),
      "2023-02-06T08:18:01.452180+00:00"
    );
  }

  #[test]
  fn test_parse_log4j_line() {
    let value = json!({
      "instant": {
        "epochSecond": 1675671481,
        "nanoOfSecond": 452180000
      },
      "thread": "Source Data Fetcher for Source: MySourceName -> *anonymous_datastream_source$4*[18] (2/2)#2798",
      "level": "INFO",
      "loggerName": "org.apache.kafka.clients.FetchSessionHandler",
      "message": "[Consumer clientId=name_72e14600-16b6-4c27-aff0-fae92ae52650-1, groupId=name_72e14600-16b6-4c27-aff0-fae92ae52650] Error sending fetch request (sessionId=1995808239, epoch=INITIAL) to node 0:",
      "thrown": {
        "commonElementCount": 0,
        "name": "org.apache.kafka.common.errors.DisconnectException"
      },
      "endOfBatch": false,
      "loggerFqcn": "org.apache.kafka.common.utils.LogContext$LocationAwareKafkaLogger",
      "threadId": 664,
      "threadPriority": 5
    });

    let actual = Log4JJsonLayoutLogLine::deserialize(value).expect("Failed to unwrap test json");
    let expected = Log4JJsonLayoutLogLine {
      thread_name: "Source Data Fetcher for Source: MySourceName -> *anonymous_datastream_source$4*[18] (2/2)#2798".to_string(),
      level: "INFO".to_string(),
      logger_name: "org.apache.kafka.clients.FetchSessionHandler".to_string(),
      end_of_batch: false,
      logger_fqcn: "org.apache.kafka.common.utils.LogContext$LocationAwareKafkaLogger".to_string(),
      message: "[Consumer clientId=name_72e14600-16b6-4c27-aff0-fae92ae52650-1, groupId=name_72e14600-16b6-4c27-aff0-fae92ae52650] Error sending fetch request (sessionId=1995808239, epoch=INITIAL) to node 0:".to_string(),
      instant: Log4JJsonLayoutLogLineInstant {
        epoch_second: 1675671481,
        nano_of_second: 452180000,
      },
      thread_id: 664,
      thread_priority: 5,
      thrown: Log4JJsonLayoutLogLineThrown {
        common_element_count: 0,
        localized_message: "".to_string(),
        message: "".to_string(),
        name: "org.apache.kafka.common.errors.DisconnectException".to_string()
      },
      mdc: Map::new(),
    };

    assert_eq!(actual, expected);
  }
}
