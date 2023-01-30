use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::TemplateAst;
use crate::err::TmplResult;
use crate::syntax::Operator;
use crate::utils::str::split_block;
use crate::TemplateConfig;

pub struct Compile {
    config: Rc<TemplateConfig>,
}

impl Compile {
    pub(crate) fn new(config: Rc<TemplateConfig>) -> Self {
        Compile { config }
    }
    pub fn build_template(&self, src: &str) -> TmplResult<Vec<TemplateAst>> {
        let result = vec![];
        let block = split_block(
            src,
            &self.config.block_symbol.0,
            &self.config.block_symbol.1,
            &vec![("'", "'"), ("\"", "\"")],
        );
        let operators = &self.config.operator;
        let start_tags = operators.iter()
            .map(|e| (e.get_start_tag(), e))
            .collect::<HashMap<&str, &Operator>>();
        let index = 0;
        loop {
            if index >= block.len() {
                break
            }

        }
        Ok(result)
    }
}
