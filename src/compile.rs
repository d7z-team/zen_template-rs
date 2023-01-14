use std::collections::HashMap;

use crate::err::TmplResult;
use crate::TemplateConfig;
use crate::utils::str::{find_block_skip_ignore, split_block};
use crate::value::TmplValue;

///表达式
#[derive(Debug)]
pub enum Expression {
    Value(TmplValue),
    Primitive(Primitive),
}

/// 原语
#[derive(Debug)]
pub struct Primitive {
    name: String,
    args: Vec<Expression>,
}

// 模板抽象语法树
pub enum TemplateAst {
    //静态字符串
    Static(String),
    // 直接变量渲染
    Expr(Expression),
    // 流程控制
    State(String, Vec<Option<Expression>>, Vec<((String, Option<Expression>), Vec<TemplateAst>)>),
    // 指令控制
    Command(String, Vec<Option<Expression>>),
}

pub struct Compile {}

impl Compile {
    pub(crate) fn build_template(src: &str, config: &TemplateConfig) -> TmplResult<TemplateAst> {
        let src_block =
            split_block(src, &config.block_symbol.0, &config.block_symbol.1, &vec![("'", "'"), ("\"", "\"")]);


        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::compile::{Expression, Primitive};
    use crate::value::TmplValue;

    #[test]
    fn test() {
        Primitive {
            name: "format".to_string(),
            args: vec![
                Expression::Primitive(Primitive {
                    name: "add".to_string(),
                    args: vec![],
                }),
                Expression::Value(TmplValue::Text("utc".to_string())),
            ],
        };
    }
}
