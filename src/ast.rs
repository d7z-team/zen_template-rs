use crate::value::TmplValue;

///表达式包装
#[derive(Debug)]
pub enum Expression {
    ///静态数据，可直接输出
    ItemStatic(TmplValue),
    ///动态表达式，需要结合原语计算
    ItemDynamic(Primitive),
}

/// 表达式转换的原语
#[derive(Debug)]
pub struct Primitive {
    ///原语名称
    name: String,
    ///原语参数
    args: Vec<Expression>,
}

impl Primitive {
    pub fn new(name: &str, args: Vec<Expression>) -> Self {
        Primitive {
            name: name.to_string(),
            args,
        }
    }
}

/// 子流程
#[derive(Debug)]
pub struct State {
    /// 流程代号
    pub key: String,
    /// 流程参数
    pub params: Vec<Expression>,
    /// 子阶段
    pub child_stage: Vec<TemplateAst>,
}

impl State {
    pub fn new(key: &str, params: Vec<Expression>) -> Self {
        State {
            key: key.to_string(),
            params,
            child_stage: vec![],
        }
    }
}

/// Easy Template 模板生成的抽象语法树
#[derive(Debug)]
pub enum TemplateAst {
    /// 变量渲染
    ItemExpr(Expression),
    /// 流程控制
    ItemState(String, Vec<State>, bool),
    /// 指令控制
    ItemCommand(String, Vec<Expression>),
}

impl TemplateAst {
    ///获取结构名称
    pub fn get_tag(&self) -> Option<&str> {
        match self {
            TemplateAst::ItemExpr(_) => None,
            TemplateAst::ItemState(e, _, _) => Some(e),
            TemplateAst::ItemCommand(e, _) => Some(e),
        }
    }
}
