#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::TemplateAst;
use crate::compile::Compile;
use crate::config::TemplateConfig;
use crate::err::{TemplateError, TmplResult};
use crate::value::TmplValue;

pub mod ast;
mod compile;
pub mod config;
mod err;
pub mod expr;
pub mod syntax;
mod template;
pub mod utils;
pub mod value;

pub type ValueFormatter = dyn Fn(&Vec<&TmplValue>) -> TmplResult<TmplValue>;

/// EasyTemplate 模板引擎
pub struct EasyTemplate {
    templates: HashMap<String, Vec<TemplateAst>>,
    config: Rc<TemplateConfig>,
    compile: Compile,
}

impl Default for EasyTemplate {
    fn default() -> Self {
        let config = Rc::new(TemplateConfig::default());
        let compile = Compile::new(Rc::clone(&config));
        EasyTemplate {
            templates: Default::default(),
            config,
            compile,
        }
    }
}

impl EasyTemplate {
    /// 注册模板
    fn register_template(&mut self, name: &str, template: &str) -> TmplResult<()> {
        let tmpl_map = &mut self.templates;
        if tmpl_map.contains_key(name) {
            // key exists.
            return Err(TemplateError::ExistsError(name.to_string()));
        }
        tmpl_map.insert(name.to_string(), self.compile.build_template(template)?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::EasyTemplate;

    #[test]
    fn test() {
        let mut template = EasyTemplate::default();
        template
            .register_template("test", include_str!("test.tmpl"))
            .unwrap();
    }
}
