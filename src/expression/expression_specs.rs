use std::collections::HashMap;

use crate::error::TmplResult;
use crate::value::TemplateValue;

///表达式处理器
pub struct ExpressionManager {
    ///符号表，包含符号转原语方式
    pub symbols: Vec<ExpressionSymbolCovert>,
    /// 原语
    pub primitive_renders: HashMap<String, PrimitiveRenderType>,
}

/// 符号转换
pub struct ExpressionSymbolCovert {
    ///符号
    pub symbol: String,
    /// 原语翻译函数
    pub covert: fn(ExpressionIR, ExpressionIR) -> ExpressionIR,
}

///原语渲染方案
pub enum PrimitiveRenderType {
    ///原语渲染：对输入的数据进行计算，并返回数据
    Native(fn(Vec<TemplateValue>) -> TmplResult<TemplateValue>),
    ///原语翻译：原语翻译，将高级原语翻译为低级原语,注意！此处不能存在
    Translate(fn(Vec<ExpressionIR>) -> TmplResult<ExpressionIR>),
}

/// 表达式编译中间代码
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionIR {
    ///标记为符号
    ItemSymbol(SymbolType),
    ///标记为最终值
    ItemValue(TemplateValue),
    ///变量
    ItemVariable(Vec<String>),
    ///原语（名称，参数）
    ItemPrimitive(String, Vec<ExpressionIR>),
    /// 一组表达式
    ItemGroup(Vec<ExpressionIR>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
///符号类型
pub enum SymbolType {
    BlockStart,
    BlockEnd,
    BlockCut,
    Custom(String),
}

///表达式
#[derive(Debug)]
pub struct Expression {
    pub variables: Vec<String>,
    pub ast: ExpressionAST,
}

///表达式 AST
#[derive(Debug)]
pub enum ExpressionAST {
    ///值
    ItemValue(TemplateValue),
    ///变量
    ItemVariable(String),
    ///原语（名称，参数）
    ItemPrimitive(String, Vec<ExpressionAST>),
}
