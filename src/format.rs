use crate::line_formats::*;
use colored::*;
use serde_json::Value;

type LogLineToColoredString = fn(&Value) -> Option<ColoredString>;

static SPECIFIC_LINE_CONVERTERS: [LogLineToColoredString; 3] = [
  ElixirLogLine::to_colored_string,
  LogstashJavaLogLine::to_colored_string,
  Log4JJsonLayoutLogLine::to_colored_string,
];

pub struct Formatter {
  last_line_converter: Option<LogLineToColoredString>,
}

impl Formatter {
  pub fn new() -> Formatter {
    Formatter {
      last_line_converter: None,
    }
  }

  pub fn format_not_json(&mut self, line: &str) -> String {
    format!("{} {}", "[NOT JSON]".red(), line.bold())
  }

  pub fn format_message(&mut self, entry: Value) -> String {
    format!("{}", self.color_format_message(entry))
  }

  fn color_format_message(&mut self, entry: Value) -> ColoredString {
    if cfg!(test) {
      println!("color_format_message");
    }
    return match self.last_line_converter {
      None => self.color_format_message_try_each(entry),
      Some(convert) => self.with_fallback(convert, entry),
    };
  }

  fn color_format_message_try_each(&mut self, entry: Value) -> ColoredString {
    if cfg!(test) {
      println!("color_format_message_try_each");
    }

    for convert in SPECIFIC_LINE_CONVERTERS.iter() {
      match convert(&entry) {
        Some(colored_string) => {
          if cfg!(test) {
            println!("Convert picked");
          }
          self.last_line_converter = Some(convert.clone());
          return colored_string;
        }
        _ => continue,
      }
    }

    if cfg!(test) {
      println!("No convert picked");
    }
    self.last_line_converter = None;
    return format_generic_json(&entry);
  }

  fn with_fallback(&mut self, convert: LogLineToColoredString, entry: Value) -> ColoredString {
    if cfg!(test) {
      println!("with_fallback");
    }
    match convert(&entry) {
      Some(colored_string) => {
        self.last_line_converter = Some(convert);
        return colored_string;
      }
      None => self.color_format_message_try_each(entry),
    }
  }
}

fn format_generic_json(entry: &Value) -> ColoredString {
  match entry {
    Value::Object(map) => {
      let meta = map
        .iter()
        .map(|(key, value)| match key.as_str() {
          "message" => "".to_string(),
          _ => format!("[{}]", text_value(value)),
        })
        .collect::<Vec<String>>()
        .join(" ");
      let text = if entry.get("message").is_some() {
        format!(" {}", &text_value(entry.get("message").unwrap()).bold())
      } else {
        "".to_string()
      };
      let level = entry.get("level").unwrap_or(&Value::Null).as_str();
      colored_with_maybe_level(level, &format!("{}{}", &meta.trim(), &text))
    }
    _ => panic!("Unsupported parsed json"),
  }
}

fn text_value(val: &Value) -> String {
  // If it's a string, show the string literal. Otherwise, render the json
  val.as_str().unwrap_or(val.to_string().as_str()).to_string()
}

fn colored_with_maybe_level(maybe_level: Option<&str>, text: &str) -> ColoredString {
  match maybe_level {
    None => text.normal(),
    Some(level) => colored_with_level(level, text),
  }
}

pub fn colored_with_level(level: &str, text: &str) -> ColoredString {
  match level.to_lowercase().as_str() {
    "info" => text.normal(),
    "warn" => text.yellow(),
    "error" => text.red(),
    "debug" => text.blue(),
    _ => text.normal(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn join(texts: Vec<ColoredString>) -> String {
    texts
      .iter()
      .fold(String::new(), |acc, text| format!("{}{}", acc, text))
  }

  fn render(text: ColoredString) -> String {
    format!("{}", text)
  }

  #[test]
  fn test_format_not_json() {
    let message = "my GenericJson message that's not JSON";
    println!("Actual: {}", Formatter::new().format_not_json(message));
    assert_eq!(
      Formatter::new().format_not_json(message),
      join(vec![
        "[NOT JSON]".red(),
        " ".normal(),
        "my GenericJson message that's not JSON".bold()
      ])
    );
  }

  #[test]
  fn test_format_minimal_working_line() {
    println!(
      "Actual: {}",
      Formatter::new().format_message(minimal_working_line())
    );
    assert_eq!(
      Formatter::new().format_message(minimal_working_line()),
      render(join(vec!["[debug] ".normal(), "My minimal working line".bold()]).blue())
    );
  }

  fn minimal_working_line() -> Value {
    json!({
      "level": "debug",
      "message": "My minimal working line"
    })
  }

  #[test]
  fn test_format_random_line() {
    println!("Actual: {}", Formatter::new().format_message(random_line()));
    assert_eq!(
      Formatter::new().format_message(random_line()),
      "[info] [This is a message] [2019-12-18T10:55:50.000345]"
    );
  }

  fn random_line() -> Value {
    json!({
      "date_time": "2019-12-18T10:55:50.000345",
      "_level": "info",
      "_message": "This is a message"
    })
  }

  #[test]
  fn test_format_elixir_line() {
    println!("Actual: {}", Formatter::new().format_message(elixir_line()));
    assert_eq!(
      Formatter::new().format_message(elixir_line()),
      join(vec![
        "[2019-12-18T10:55:50.000345] [info] [ecto_sql] [Elixir.Ecto.Migration.Runner] [#PID<0.274.0>] ".normal(),
        "== Migrated 123456789 in 0.0s".bold()
      ])
    );
  }

  fn elixir_line() -> Value {
    json!({
      "app": "ecto_sql",
      "level": "info",
      "message": "== Migrated 123456789 in 0.0s",
      "metadata": {},
      "module": "Elixir.Ecto.Migration.Runner",
      "pid": "#PID<0.274.0>",
      "timestamp": "2019-12-18T10:55:50.000345"
    })
  }

  #[test]
  fn test_format_logstash_java_line() {
    println!(
      "Actual: {}",
      Formatter::new().format_message(logstash_java_line())
    );
    assert_eq!(
      Formatter::new().format_message(logstash_java_line()),
      render(join(vec![
        "[2020-01-13T12:34:01.740Z] [DEBUG] [org.apache.flink.runtime.dispatcher.StandaloneDispatcher] [flink-akka.actor.default-dispatcher-3] ".normal(),
        "Dispatcher akka.tcp://flink@04fc4fd30dc3:6123/user/dispatcher accepted leadership with fencing token 00000000000000000000000000000000. Start recovered jobs.".bold()
      ]).blue())
    );
  }

  fn logstash_java_line() -> Value {
    json!({
      "@timestamp": "2020-01-13T12:34:01.740Z",
      "source_host": "04fc4fd30dc3",
      "file": "Dispatcher.java",
      "method": "tryAcceptLeadershipAndRunJobs",
      "level": "DEBUG",
      "line_number": "927",
      "thread_name": "flink-akka.actor.default-dispatcher-3",
      "@version": 1,
      "logger_name": "org.apache.flink.runtime.dispatcher.StandaloneDispatcher",
      "message": "Dispatcher akka.tcp://flink@04fc4fd30dc3:6123/user/dispatcher accepted leadership with fencing token 00000000000000000000000000000000. Start recovered jobs.",
      "class": "org.apache.flink.runtime.dispatcher.Dispatcher",
      "mdc": {}
    })
  }

  #[test]
  fn test_format_different_lines() {
    let mut formatter = Formatter::new();

    assert!(formatter.last_line_converter.is_none());

    assert_eq!(
      formatter.format_message(logstash_java_line()),
      render(join(vec![
        "[2020-01-13T12:34:01.740Z] [DEBUG] [org.apache.flink.runtime.dispatcher.StandaloneDispatcher] [flink-akka.actor.default-dispatcher-3] ".normal(),
        "Dispatcher akka.tcp://flink@04fc4fd30dc3:6123/user/dispatcher accepted leadership with fencing token 00000000000000000000000000000000. Start recovered jobs.".bold()
      ]).blue())
    );

    assert!(formatter.last_line_converter.is_some());

    assert_eq!(
      formatter.format_message(elixir_line()),
      join(vec![
        "[2019-12-18T10:55:50.000345] [info] [ecto_sql] [Elixir.Ecto.Migration.Runner] [#PID<0.274.0>] ".normal(),
        "== Migrated 123456789 in 0.0s".bold()
      ])
    );

    assert!(formatter.last_line_converter.is_some());

    assert_eq!(
      formatter.format_message(random_line()),
      "[info] [This is a message] [2019-12-18T10:55:50.000345]"
    );

    assert!(formatter.last_line_converter.is_none());

    assert_eq!(
      formatter.format_message(elixir_line()),
      join(vec![
        "[2019-12-18T10:55:50.000345] [info] [ecto_sql] [Elixir.Ecto.Migration.Runner] [#PID<0.274.0>] ".normal(),
        "== Migrated 123456789 in 0.0s".bold()
      ])
    );

    assert!(formatter.last_line_converter.is_some());

    assert_eq!(
      formatter.format_message(logstash_java_line()),
      render(join(vec![
        "[2020-01-13T12:34:01.740Z] [DEBUG] [org.apache.flink.runtime.dispatcher.StandaloneDispatcher] [flink-akka.actor.default-dispatcher-3] ".normal(),
        "Dispatcher akka.tcp://flink@04fc4fd30dc3:6123/user/dispatcher accepted leadership with fencing token 00000000000000000000000000000000. Start recovered jobs.".bold()
      ]).blue())
    );

    assert!(formatter.last_line_converter.is_some());
  }

  #[test]
  fn test_format_log4j_json_layout_java_line() {
    println!(
      "Actual: {}",
      Formatter::new().format_message(log4j_json_layout_java_line())
    );
    assert_eq!(
      Formatter::new().format_message(log4j_json_layout_java_line()),
      render(join(vec![
        "[2021-06-03T12:50:07.420+00:00] [WARN] [org.apache.flink.runtime.dispatcher.DispatcherRestEndpoint] [main] ".normal(),
        "Starting rest endpoint.".bold()
      ]).yellow())
    );
  }

  fn log4j_json_layout_java_line() -> Value {
    json!({
      "thread": "main",
      "level": "WARN",
      "loggerName": "org.apache.flink.runtime.dispatcher.DispatcherRestEndpoint",
      "message": "Starting rest endpoint.",
      "endOfBatch": false,
      "loggerFqcn": "org.apache.logging.slf4j.Log4jLogger",
      "instant": {
        "epochSecond": 1622724607,
        "nanoOfSecond": 420000000
      },
      "threadId": 1,
      "threadPriority": 5
    })
  }
}
