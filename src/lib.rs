#![allow(dead_code)]

use std::collections::HashMap;
use crate::compile::{Compile, TemplateAst};

use crate::err::{TemplateError, TmplResult};
use crate::value::TmplValue;

mod err;
mod value;
mod template;
mod compile;
mod syntax;
mod utils;

pub type ValueFormatter = dyn Fn(&Vec<&TmplValue>) -> TmplResult<TmplValue>;

/// EasyTemplate 模板引擎
pub struct EasyTemplate {
    templates: HashMap<String, TemplateAst>,
    config: TemplateConfig,
}

/// 模板配置项
pub struct TemplateConfig {
    /// 原语
    primitives: HashMap<String, HashMap<usize, Box<ValueFormatter>>>,
    /// 符号表达式渲染规则
    expressions_symbol: Vec<(String, String)>,
}


impl EasyTemplate {
    /// 注册模板
    fn register_template(&mut self, name: &str, template: &str) -> TmplResult<()> {
        let tmpls = &mut self.templates;
        if tmpls.contains_key(name) {
            // key exists.
            return Err(TemplateError::ExistsError(name.to_string()));
        }
        tmpls.insert(name.to_string(),
                     Compile::build_template(template, &self.config)?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
