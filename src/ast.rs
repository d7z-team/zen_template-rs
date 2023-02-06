use crate::ast::TemplateAst::*;
use crate::expr::common::ExpressionIR;
use crate::expr::common::ExpressionIR::ItemValue;
use crate::value::TmplValue;

/// 参数包装
#[derive(Debug)]
pub enum CommandParam {
    Keywords,
    Assignment(Vec<String>),
    Expression(ExpressionIR),
    StaticValue(TmplValue),
}

/// 子流程
#[derive(Debug)]
pub struct Branch {
    /// 流程代号
    pub key: String,
    /// 流程参数
    pub params: Vec<CommandParam>,
    /// 子阶段
    pub child_stages: Vec<TemplateAst>,
}

impl Branch {
    pub fn new(key: &str, params: Vec<CommandParam>) -> Self {
        Branch {
            key: key.to_string(),
            params,
            child_stages: vec![],
        }
    }
}

/// Easy Template 模板生成的抽象语法树
#[derive(Debug)]
pub enum TemplateAst {
    /// 变量渲染，属于控制对象
    ItemExpr(ExpressionIR),
    /// 流程控制，属于分支对象
    ItemBranch(String, Vec<Branch>, bool),
    /// 指令控制,属于控制对象
    ItemCommand(String, Vec<CommandParam>),
}

impl TemplateAst {
    ///获取结构名称
    pub fn get_tag(&self) -> Option<&str> {
        match self {
            ItemExpr(_) => None,
            ItemBranch(e, _, _) => Some(e),
            ItemCommand(e, _) => Some(e),
        }
    }
    pub fn new_text(text: &str) -> TemplateAst {
        ItemExpr(ItemValue(TmplValue::Text(text.to_string())))
    }
}
