use crate::ast::{Expression, Primitive};
use std::collections::HashMap;

/// 符号注册
pub struct ExprSymbol {
    //符号
    symbol: String,
    // 原语翻译函数
    covert: fn(Expression, Expression) -> Primitive,
}

/// 表达式符号映射表，将相关的表达式转换为原语
pub fn default_expressions_symbol() -> Vec<ExprSymbol> {
    let mut result = Vec::new();
    let mut register = |tag: &str, evolution: fn(Expression, Expression) -> Primitive| {
        result.push(ExprSymbol {
            symbol: tag.to_string(),
            covert: evolution,
        })
    };
    register(".", |e, a| Primitive::new("get", vec![e, a]));
    register("?:", |e, a| Primitive::new("get_or_default", vec![e, a]));
    register("?.", |e, a| Primitive::new("get_or_none", vec![e, a]));

    register("*", |e, a| Primitive::new("multi", vec![e, a]));
    register("/", |e, a| Primitive::new("div", vec![e, a]));
    register("%", |e, a| Primitive::new("mod", vec![e, a]));

    register("+", |e, a| Primitive::new("add", vec![e, a]));
    register("-", |e, a| Primitive::new("sub", vec![e, a]));

    register(" is ", |e, a| {
        Primitive::new(
            "eq",
            vec![Expression::Dynamic(Primitive::new("type", vec![e])), a],
        )
    });
    register("==", |e, a| Primitive::new("eq", vec![e, a]));
    register("!=", |e, a| {
        Primitive::new(
            "not",
            vec![Expression::Dynamic(Primitive::new("eq", vec![e, a]))],
        )
    });
    register(">=", |e, a| Primitive::new("ge", vec![e, a]));
    register("<=", |e, a| Primitive::new("le", vec![e, a]));
    register(">", |e, a| Primitive::new("r_angle", vec![e, a]));
    register("<", |e, a| Primitive::new("l_angle", vec![e, a]));
    register("&&", |e, a| Primitive::new("and", vec![e, a]));
    register("||", |e, a| Primitive::new("or", vec![e, a]));

    result
}

// 流程控制参数
pub enum ParamSyntax {
    /// 关键字标记
    Keywords(String),
    /// 赋值标记
    Assignment,
    /// 表达式标记
    Expression,
    /// 静态数据标记，
    StaticValue,
}

/// 流程控制关键字和语法标记
pub struct OperatorTag {
    // 操作符标记
    tag: String,
    // 操作符语法 (名称 / 表达式)
    syntax: HashMap<String, Vec<ParamSyntax>>,
}

impl OperatorTag {
    fn new(tag: &str, syntax: Vec<(&str, Vec<ParamSyntax>)>) -> Self {
        OperatorTag {
            tag: tag.to_string(),
            syntax: syntax
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }
    }
}

pub enum ChildStageType {
    Single(OperatorTag),
    Multiple(OperatorTag),
}

/// 带子流程的操作运算
pub struct OperatorBlock {
    /// 开始关键字
    start: OperatorTag,
    /// 子分支
    child_state: Vec<ChildStageType>,
    /// 结束关键字
    end: OperatorTag,
}

impl OperatorBlock {
    fn new(start: OperatorTag, child: Vec<ChildStageType>, end: OperatorTag) -> Self {
        OperatorBlock {
            start,
            child_state: child,
            end,
        }
    }
}

/// 流程操作符分类
pub enum Operator {
    /// 流程分支
    Branch(OperatorBlock, Vec<String>, bool),
    /// 流程控制命令
    Command(OperatorTag, Vec<String>),
}

impl Operator {
    pub fn new_branch(block: OperatorBlock, scope: Vec<&str>, loop_state: bool) -> Self {
        Operator::Branch(
            block,
            scope.iter().map(|e| e.to_string()).collect(),
            loop_state,
        )
    }
    pub fn new_command(tag: OperatorTag, scope: Vec<&str>) -> Self {
        Operator::Command(tag, scope.iter().map(|e| e.to_string()).collect())
    }
    pub(crate) fn get_start_tag(&self) -> &str {
        match self {
            Operator::Branch(tag, _, _) => &tag.start.tag,
            Operator::Command(tag, _) => &tag.tag,
        }
    }
}

pub fn default_state() -> Vec<Operator> {
    let mut result = Vec::new();
    result.push(Operator::new_branch(
        OperatorBlock::new(
            OperatorTag::new(
                "for",
                vec![(
                    "default",
                    vec![
                        ParamSyntax::Assignment,
                        ParamSyntax::Keywords("in".to_string()),
                        ParamSyntax::Expression,
                    ],
                )],
            ),
            vec![],
            OperatorTag::new("end-for", vec![]),
        ),
        vec![],
        true,
    ));
    result.push(Operator::new_branch(
        OperatorBlock::new(
            OperatorTag::new("switch", vec![("default", vec![ParamSyntax::Expression])]),
            vec![
                ChildStageType::Multiple(OperatorTag::new(
                    "case",
                    vec![("default", vec![ParamSyntax::StaticValue])],
                )),
                ChildStageType::Single(OperatorTag::new("default", vec![])),
            ],
            OperatorTag::new("end-switch", vec![]),
        ),
        vec![],
        false,
    ));
    //if
    result.push(Operator::new_branch(
        OperatorBlock::new(
            OperatorTag::new(
                "if",
                vec![
                    ("default", vec![ParamSyntax::Expression]),
                    (
                        "let",
                        vec![ParamSyntax::Assignment, ParamSyntax::Expression],
                    ),
                ],
            ),
            vec![
                ChildStageType::Multiple(OperatorTag::new(
                    "else-if",
                    vec![
                        ("default", vec![ParamSyntax::Expression]),
                        (
                            "let",
                            vec![ParamSyntax::Assignment, ParamSyntax::Expression],
                        ),
                    ],
                )),
                ChildStageType::Single(OperatorTag::new("else", vec![])),
            ],
            OperatorTag::new("end-if", vec![]),
        ),
        vec![],
        false,
    ));
    result.push(Operator::new_command(
        OperatorTag::new("include", vec![("default", vec![ParamSyntax::Expression])]),
        vec![],
    ));
    result.push(Operator::new_command(
        OperatorTag::new(
            "let",
            vec![(
                "default",
                vec![ParamSyntax::Assignment, ParamSyntax::Expression],
            )],
        ),
        vec![],
    ));
    result.push(Operator::new_command(
        OperatorTag::new("break", vec![]),
        vec!["loop", "for"],
    ));
    result.push(Operator::new_command(
        OperatorTag::new("continue", vec![]),
        vec!["loop", "for"],
    ));
    result
}
