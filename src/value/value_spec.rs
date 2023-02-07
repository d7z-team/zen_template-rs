use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TemplateValue {
    Float(f64),
    Number(i64),
    Text(String),
    Bool(bool),
    Array(Vec<TemplateValue>),
    Table(HashMap<String, TemplateValue>),
    //仅用于参与计算
    None,
}
