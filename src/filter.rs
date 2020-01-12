use regex::Regex;
use serde_json::Value;

pub fn parse_filters(unparsed_filters: Vec<&str>) -> Vec<Filter> {
  unparsed_filters
    .iter()
    .map(|t| *t)
    .map(Filter::from)
    .collect()
}

pub fn passes_filters(filters: &Vec<Filter>, entry: &Value) -> bool {
  filters.iter().all(|f| f.passes(entry))
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum FilterKind {
  Equals,
  Contains,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Filter {
  key: String,
  kind: FilterKind,
  value: String,
}

impl Filter {
  fn passes(&self, entry: &Value) -> bool {
    let possible_value = entry.get(&self.key);
    if possible_value.is_none() {
      false
    } else {
      let value = possible_value.unwrap().as_str().unwrap();
      match self.kind {
        FilterKind::Equals => value == self.value,
        FilterKind::Contains => value.contains(self.value.as_str())
      }
    }
  }

  fn equals(key: &str, value: &str) -> Self {
    Filter {
      kind: FilterKind::Equals,
      key: key.to_string(),
      value: value.to_string(),
    }
  }

  fn contains(key: &str, value: &str) -> Self {
    Filter {
      kind: FilterKind::Contains,
      key: key.to_string(),
      value: value.to_string(),
    }
  }

  fn from(text: &str) -> Filter {
    lazy_static! {
        static ref CONTAINS_REGEX: Regex = Regex::new(r"^([^=]+)=\+([^=]+)$").unwrap();
        static ref EQUALS_REGEX: Regex = Regex::new(r"^([^=]+)=([^=]+)").unwrap();
    }

    if CONTAINS_REGEX.is_match(text) {
      let caps = CONTAINS_REGEX.captures(text).unwrap();
      Filter::contains(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str())
    } else if EQUALS_REGEX.is_match(text) {
      let caps = EQUALS_REGEX.captures(text).unwrap();
      Filter::equals(caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str())
    } else {
      println!("Can't parse filter: {}", text);
      panic!("Error.");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_filter_equal() {
    assert_eq!(
      Filter::from("mykey=m'value"),
      Filter::equals("mykey", "m'value")
    );
  }

  #[test]
  fn test_parse_filter_contains() {
    assert_eq!(
      Filter::from("the_key=+search_for"),
      Filter::contains("the_key", "search_for"),
    );
  }

  #[test]
  fn test_parse_filter_list() {
    assert_eq!(
      parse_filters(vec!["the_key=+search_for", "this=that", "module=+Drive"]),
      vec![
        Filter::contains("the_key", "search_for"),
        Filter::equals("this", "that"),
        Filter::contains("module", "Drive")
      ]
    );
  }

  #[test]
  fn filter_equal_passes() {
    assert!(Filter::equals("app", "drive").passes(&build_line()));
    assert!(!Filter::equals("app", "riv").passes(&build_line()));
    assert!(!Filter::equals("app", "test").passes(&build_line()));
  }

  #[test]
  fn filter_contains_passes() {
    assert!(!Filter::contains("app", "de").passes(&build_line()));
    assert!(Filter::contains("app", "riv").passes(&build_line()));
  }

  #[test]
  fn pass_all_filters() {
    assert!(passes_filters(&vec![
      Filter::contains("module", "Flink"),
      Filter::equals("app", "drive")
    ], &build_line()));

    assert!(!passes_filters(&vec![
      Filter::contains("module", "Kafka"),
      Filter::equals("app", "drive")
    ], &build_line()));

    assert!(!passes_filters(&vec![
      Filter::equals("module", "Flink"),
      Filter::equals("app", "drive")
    ], &build_line()));
  }

  #[test]
  fn parse_and_pass_filters() {
    assert!(
      passes_filters(
        &parse_filters(vec!["app=+drive", "module=+Flink"]),
        &build_line(),
      )
    );
    assert!(
      !passes_filters(
        &parse_filters(vec!["app=operate", "module=+Flink"]),
        &build_line(),
      )
    );
  }

  fn build_line() -> Value {
    json!({ "app": "drive", "module": "Elixir.Drive.FlinkJob" })
  }
}
