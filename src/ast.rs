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

/// 流程下的每个流程
#[derive(Debug)]
pub struct State {
    name: String,
    params: Vec<Expression>,
    child: Vec<TemplateAst>,
}

/// Easy Template 模板生成的抽象语法树
#[derive(Debug)]
pub enum TemplateAst {
    ///静态字符串渲染
    Static(String),
    /// 变量渲染
    Expr(Expression),
    /// 流程控制
    State(String, Vec<State>),
    /// 循环控制
    Loop(Vec<TemplateAst>),
    /// 指令控制
    Command(String, Vec<Expression>),
}

#[cfg(test)]
mod test {
    use crate::ast::{Expression, Primitive};
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
