#![allow(dead_code)]

use std::collections::HashMap;

use crate::err::TmplResult;
use crate::value::Value::{Bool, Float, Number, Text};

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Number(i64),
    Text(String),
    Bool(bool),
    Array(Vec<Value>),
    Table(HashMap<String, Value>),
    None,
}

impl Value {
    fn from(src: &str) -> TmplResult<Value> {
        if src == "None" {
            return Ok(Value::None)
        }
        src.parse::<bool>().map(|e| Bool(e))
            .or_else(|_| src.parse::<i64>().map(|e| Number(e)))
            .or_else(|_| src.parse::<f64>().map(|e| Float(e)))
            .or_else(|_| Ok(Text(src.to_string())))
    }
}

#[test]
fn test() {
    println!("{:?}", Value::from("false"));
    println!("{:?}", Value::from("12.33"));
    println!("{:?}", Value::from("15"));
    println!("{:?}", Value::from("text"));
}
