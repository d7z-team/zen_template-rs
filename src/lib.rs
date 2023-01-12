#![allow(dead_code)]

use std::collections::HashMap;

use crate::err::TmplResult;
use crate::value::Value;

mod err;
mod value;
mod template;

pub type ValueFormatter = dyn Fn(&Vec<&Value>) -> TmplResult<Value>;

pub struct EasyTemplate<'tmpl> {
    templates: HashMap<&'tmpl str, String>,
    formatters: HashMap<&'tmpl str, HashMap<usize, Box<ValueFormatter>>>,
}
