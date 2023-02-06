use std::collections::HashMap;

use crate::err::TmplResult;
use crate::expr::{ExpressionIR, Primitive};
use crate::template::default_expressions_symbol;
use crate::value::TmplValue;

///表达式处理器
pub struct ExpressionManager {
    ///符号表，包含符号转原语方式
    pub(crate) symbols: Vec<ExprSymbolCovert>,
    /// 原语
    pub(crate) primitive_renders: HashMap<String, PrimitiveRenderType>,
}

/// 符号转换
pub struct ExprSymbolCovert {
    ///符号
    pub symbol: String,
    /// 原语翻译函数
    pub covert: fn(ExpressionIR, ExpressionIR) -> ExpressionIR,
}

///原语渲染方案
pub enum PrimitiveRenderType {
    ///原语渲染：对输入的数据进行计算，并返回数据
    Native(fn(Vec<TmplValue>) -> TmplResult<TmplValue>),
    ///原语翻译：原语翻译，将高级原语翻译为低级原语
    Translate(fn(Vec<ExpressionIR>) -> TmplResult<Primitive>),
}

impl Default for ExpressionManager {
    fn default() -> Self {
        ExpressionManager {
            symbols: default_expressions_symbol(),
            primitive_renders: HashMap::new(),
        }
    }
}
