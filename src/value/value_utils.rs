#[cfg(any(feature = "yaml", test))]
use std::ops::Not;

#[cfg(any(feature = "json", test))]
use serde_json::Value as JsonValue;
#[cfg(any(feature = "yaml", test))]
use serde_yaml::Value as YamlValue;

#[cfg(any(feature = "json", test))]
use crate::error::TemplateError;
#[cfg(any(feature = "serde", test))]
use crate::error::TmplResult;
use crate::value::TemplateValue;

impl PartialEq for TemplateValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TemplateValue::Float(_) => false,
            TemplateValue::Number(e) => {
                if let TemplateValue::Number(oe) = other {
                    e.eq(oe)
                } else {
                    false
                }
            }
            TemplateValue::Text(e) => {
                if let TemplateValue::Text(oe) = other {
                    e.eq(oe)
                } else {
                    false
                }
            }
            TemplateValue::Bool(e) => {
                if let TemplateValue::Bool(oe) = other {
                    e.eq(oe)
                } else {
                    false
                }
            }
            TemplateValue::Array(e) => {
                if let TemplateValue::Array(oe) = other {
                    e.eq(oe)
                } else {
                    false
                }
            }
            TemplateValue::Table(e) => {
                if let TemplateValue::Table(oe) = other {
                    e.eq(oe)
                } else {
                    false
                }
            }
            TemplateValue::None => {
                if let TemplateValue::None = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl ToString for TemplateValue {
    fn to_string(&self) -> String {
        match self {
            TemplateValue::Float(f) => f.to_string(),
            TemplateValue::Number(n) => n.to_string(),
            TemplateValue::Text(t) => t.to_string(),
            TemplateValue::Bool(b) => b.to_string(),
            TemplateValue::Array(a) => a
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
            TemplateValue::Table(t) => t
                .iter()
                .map(|e| format!("{}={}", e.0.to_string(), e.1.to_string()))
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
            TemplateValue::None => "None".to_string(),
        }
    }
}

impl TemplateValue {
    pub fn from(src: &str) -> TemplateValue {
        src.parse::<bool>()
            .map(|e| TemplateValue::Bool(e))
            .or_else(|_| src.parse::<i64>().map(|e| TemplateValue::Number(e)))
            .or_else(|_| src.parse::<f64>().map(|e| TemplateValue::Float(e)))
            .unwrap_or(TemplateValue::Text(src.to_string()))
    }

    #[cfg(any(feature = "json", test))]
    pub fn from_json(src: &str) -> TmplResult<TemplateValue> {
        let value: JsonValue =
            serde_json::from_str(src).map_err(|e| TemplateError::GenericError(e.to_string()))?;
        fn covert_value(old: &JsonValue) -> TemplateValue {
            match old {
                JsonValue::Null => TemplateValue::None,
                JsonValue::Bool(item) => TemplateValue::Bool(item.clone()),
                JsonValue::Number(item) => {
                    if item.is_f64() {
                        TemplateValue::Float(item.as_f64().unwrap())
                    } else {
                        TemplateValue::Number(item.as_i64().unwrap())
                    }
                }
                JsonValue::String(item) => TemplateValue::Text(item.to_string()),
                JsonValue::Array(array) => TemplateValue::Array(
                    array
                        .iter()
                        .map(|e| covert_value(e))
                        .collect::<Vec<TemplateValue>>(),
                ),
                JsonValue::Object(object) => TemplateValue::Table(
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
    pub fn from_yaml(src: &str) -> TmplResult<TemplateValue> {
        let value: YamlValue =
            serde_yaml::from_str(src).map_err(|e| TemplateError::GenericError(e.to_string()))?;
        fn covert_value(old: &YamlValue) -> TemplateValue {
            match old {
                YamlValue::Null => TemplateValue::None,
                YamlValue::Bool(item) => TemplateValue::Bool(item.clone()),
                YamlValue::Number(item) => {
                    if item.is_f64().not() {
                        TemplateValue::Number(item.as_i64().unwrap())
                    } else {
                        TemplateValue::Float(item.as_f64().unwrap())
                    }
                }
                YamlValue::String(item) => TemplateValue::Text(item.to_string()),
                YamlValue::Sequence(array) => TemplateValue::Array(
                    array
                        .iter()
                        .map(|e| covert_value(e))
                        .collect::<Vec<TemplateValue>>(),
                ),
                YamlValue::Mapping(object) => TemplateValue::Table(
                    object
                        .iter()
                        .map(|(k, v)| (covert_value(k).to_string(), covert_value(v)))
                        .collect(),
                ),
                YamlValue::Tagged(_) => TemplateValue::None,
            }
        }
        Ok(covert_value(&value))
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::value::TemplateValue;

    #[test]
    fn test_from() {
        assert_eq!(
            TemplateValue::from("false").to_string(),
            TemplateValue::Bool(false).to_string()
        );
        assert_eq!(
            TemplateValue::from("12.33").to_string(),
            TemplateValue::Float(12.33).to_string()
        );
        assert_eq!(
            TemplateValue::from("15").to_string(),
            TemplateValue::Number(15).to_string()
        );
        assert_eq!(
            TemplateValue::from("text").to_string(),
            TemplateValue::Text("text".to_string()).to_string()
        );
    }

    #[test]
    fn test_from_json() {
        assert!(TemplateValue::from_json(
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
        assert!(TemplateValue::from_yaml(yaml).is_ok())
    }
}
