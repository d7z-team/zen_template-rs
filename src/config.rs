use std::collections::HashMap;

use crate::expr::{ExprSymbolCovert, ExpressionManager};
use crate::syntax::OperatorSyntax;
use crate::template::{default_expressions_symbol, default_state};
use crate::ValueFormatter;

/// 模板配置项
pub struct TemplateConfig {
    /// 原语
    pub primitives: HashMap<String, HashMap<usize, Box<ValueFormatter>>>,
    /// 符号表达式渲染规则
    pub expr_manager: ExpressionManager,
    /// 模板块符号
    pub block_symbol: (String, String),
    /// 流程控制符号
    pub operators: Vec<OperatorSyntax>,
}

impl TemplateConfig {
    ///根据原始标记查询对应的匹配语法
    ///
    pub fn get_operator_by_start(&self, src: &str) -> Option<&OperatorSyntax> {
        self.operators
            .iter()
            .find(|item| src.starts_with(&format!("{} ", item.get_start_tag())))
    }
}

#[cfg(any(test))]
pub(crate) fn init_log() {
    let _ = simple_logger::init_with_level(log::Level::Debug);
}

#[cfg(not(test))]
pub(crate) fn init_log() {}

impl Default for TemplateConfig {
    fn default() -> Self {
        init_log();
        TemplateConfig {
            primitives: HashMap::new(),
            expr_manager: ExpressionManager::default(),
            block_symbol: (String::from("{{"), String::from("}}")),
            operators: default_state(),
        }
    }
}
