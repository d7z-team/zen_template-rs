#![allow(dead_code)]

use std::collections::HashMap;

use crate::compile::{Compile, TemplateAst};
use crate::err::{TemplateError, TmplResult};
use crate::syntax::{default_expressions_symbol, default_state, Operator};
use crate::value::TmplValue;

mod err;
mod value;
mod template;
mod compile;
pub mod syntax;
pub mod utils;

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
    /// 模板块符号
    block_symbol: (String, String),
    /// 流程控制符号
    operator: Vec<Operator>,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        TemplateConfig {
            primitives: HashMap::new(),
            expressions_symbol: default_expressions_symbol(),
            block_symbol: (String::from("{{"), String::from("}}")),
            operator: default_state(),
        }
    }
}

impl EasyTemplate {
    pub fn new() -> Self {
        EasyTemplate { templates: Default::default(), config: Default::default() }
    }
    /// 注册模板
    fn register_template(&mut self, name: &str, template: &str) -> TmplResult<()> {
        let tmpl_map = &mut self.templates;
        if tmpl_map.contains_key(name) {
            // key exists.
            return Err(TemplateError::ExistsError(name.to_string()));
        }
        tmpl_map.insert(name.to_string(),
                        Compile::build_template(template, &self.config)?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::EasyTemplate;

    #[test]
    fn test() {
        let mut template = EasyTemplate::new();
        template.register_template("test", include_str!("test.tmpl")).unwrap();
    }
}
