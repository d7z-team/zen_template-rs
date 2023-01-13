#![allow(dead_code)]

use std::collections::HashMap;

use crate::err::TmplResult;
use crate::value::TmplValue;

mod err;
mod value;
mod template;
mod compile;
mod syntax;

pub type ValueFormatter = dyn Fn(&Vec<&TmplValue>) -> TmplResult<TmplValue>;


pub struct EasyTemplate<'tmpl> {
    templates: HashMap<&'tmpl str, String>,
    primitives: HashMap<String, HashMap<usize, Box<ValueFormatter>>>,
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
