#![allow(dead_code)]

use std::collections::HashMap;
#[cfg(any(feature = "yaml", test))]
use std::ops::Not;

#[cfg(any(feature = "json", test))]
use serde_json::Value as JsonValue;
#[cfg(any(feature = "yaml", test))]
use serde_yaml::Value as YamlValue;

#[cfg(any(feature = "json", test))]
use crate::err::TemplateError;
use crate::err::TmplResult;

#[derive(Debug, Clone)]
pub enum TmplValue {
    Float(f64),
    Number(i64),
    Text(String),
    Bool(bool),
    Array(Vec<TmplValue>),
    Table(HashMap<String, TmplValue>),
    None,
}

impl ToString for TmplValue {
    fn to_string(&self) -> String {
        match self {
            TmplValue::Float(f) => f.to_string(),
            TmplValue::Number(n) => n.to_string(),
            TmplValue::Text(t) => t.to_string(),
            TmplValue::Bool(b) => b.to_string(),
            TmplValue::Array(a) => a
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
            TmplValue::Table(t) => t
                .iter()
                .map(|e| format!("{}={}", e.0.to_string(), e.1.to_string()))
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
            TmplValue::None => "None".to_string(),
        }
    }
}

impl TmplValue {
    pub fn from(src: &str) -> TmplResult<TmplValue> {
        src.parse::<bool>()
            .map(|e| TmplValue::Bool(e))
            .or_else(|_| src.parse::<i64>().map(|e| TmplValue::Number(e)))
            .or_else(|_| src.parse::<f64>().map(|e| TmplValue::Float(e)))
            .or_else(|_| Ok(TmplValue::Text(src.to_string())))
    }

    #[cfg(any(feature = "json", test))]
    pub fn from_json(src: &str) -> TmplResult<TmplValue> {
        let value: JsonValue =
            serde_json::from_str(src).map_err(|e| TemplateError::GenericError(e.to_string()))?;
        fn covert_value(old: &JsonValue) -> TmplValue {
            match old {
                JsonValue::Null => TmplValue::None,
                JsonValue::Bool(item) => TmplValue::Bool(item.clone()),
                JsonValue::Number(item) => {
                    if item.is_f64() {
                        TmplValue::Float(item.as_f64().unwrap())
                    } else {
                        TmplValue::Number(item.as_i64().unwrap())
                    }
                }
                JsonValue::String(item) => TmplValue::Text(item.to_string()),
                JsonValue::Array(array) => TmplValue::Array(
                    array
                        .iter()
                        .map(|e| covert_value(e))
                        .collect::<Vec<TmplValue>>(),
                ),
                JsonValue::Object(object) => TmplValue::Table(
                    object
                        .iter()
                        .map(|(k, v)| (k.to_string(), covert_value(v)))
                        .collect(),
                ),
            }
        }
        Ok(covert_value(&value))
    }
    #[cfg(any(feature = "yaml", test))]
    pub fn from_yaml(src: &str) -> TmplResult<TmplValue> {
        let value: YamlValue =
            serde_yaml::from_str(src).map_err(|e| TemplateError::GenericError(e.to_string()))?;
        fn covert_value(old: &YamlValue) -> TmplValue {
            match old {
                YamlValue::Null => TmplValue::None,
                YamlValue::Bool(item) => TmplValue::Bool(item.clone()),
                YamlValue::Number(item) => {
                    if item.is_f64().not() {
                        TmplValue::Number(item.as_i64().unwrap())
                    } else {
                        TmplValue::Float(item.as_f64().unwrap())
                    }
                }
                YamlValue::String(item) => TmplValue::Text(item.to_string()),
                YamlValue::Sequence(array) => TmplValue::Array(
                    array
                        .iter()
                        .map(|e| covert_value(e))
                        .collect::<Vec<TmplValue>>(),
                ),
                YamlValue::Mapping(object) => TmplValue::Table(
                    object
                        .iter()
                        .map(|(k, v)| (covert_value(k).to_string(), covert_value(v)))
                        .collect(),
                ),
                YamlValue::Tagged(_) => TmplValue::None,
            }
        }
        Ok(covert_value(&value))
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::value::TmplValue;

    #[test]
    fn test_from() {
        assert_eq!(
            TmplValue::from("false").unwrap().to_string(),
            TmplValue::Bool(false).to_string()
        );
        assert_eq!(
            TmplValue::from("12.33").unwrap().to_string(),
            TmplValue::Float(12.33).to_string()
        );
        assert_eq!(
            TmplValue::from("15").unwrap().to_string(),
            TmplValue::Number(15).to_string()
        );
        assert_eq!(
            TmplValue::from("text").unwrap().to_string(),
            TmplValue::Text("text".to_string()).to_string()
        );
    }

    #[test]
    fn test_from_json() {
        assert!(TmplValue::from_json(
            r#"
         {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
        "#
        )
        .is_ok());
    }

    #[test]
    fn test_from_yaml() {
        let yaml = indoc! {"
        ---
        - &CENTER { x: 1, y: 2 }
        - &LEFT { x: 0, y: 2 }
        - &BIG { r: 10 }
        - &SMALL { r: 1 }
        # All the following maps are equal:
        - # Explicit keys
          x: 1
          y: 2
          r: 10
          label: center/big
        - # Merge one map
          << : *CENTER
          r: 10
          label: center/big
        - # Merge multiple maps
          << : [ *CENTER, *BIG ]
          label: center/big
        - # Override
          << : [ *BIG, *LEFT, *SMALL ]
          x: 1
          label: center/big
        "};
        assert!(TmplValue::from_yaml(yaml).is_ok())
    }
}
