use std::collections::HashMap;

/// 表达式符号映射表，将相关的表达式转换为原语
pub fn default_expressions_symbol() -> Vec<(String, String)> {
    let mut syn_map = Vec::new();
    let mut register = |tag: &str, evolution: &str| {
        syn_map.push((tag.to_string(), evolution.to_string()))
    };
    register(".", "get($left,$right)");
    register("?:", "get_or_default($left,$right)");
    register("?.", "get_or_none($left,$right)");

    register("*", "multi($left,$right)");
    register("/", "div($left,$right)");
    register("%", "mod($left,$right)");

    register("+", "add($left,$right)");
    register("-", "sub($left,$right)");

    register(" is ", "eq(type($left),$right)");
    register("==", "eq($left,$right)");
    register("!=", "not_eq($left,$right)");
    register(">=", "ge($left,$right)");
    register("<=", "le($left,$right)");
    register(">", "r_angle($left,$right)");
    register("<", "l_angle($left,$right)");

    register("&&", "and($left,$right)");
    register("||", "or($left,$right)");
    syn_map
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
            syntax: syntax.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
        }
    }
}

pub enum ChildStateType {
    Single(OperatorTag),
    Multiple(OperatorTag),
}

/// 带子流程的操作运算
pub struct OperatorBlock {
    /// 开始关键字
    start: OperatorTag,
    /// 子分支
    child_state: Vec<ChildStateType>,
    /// 结束关键字
    end: OperatorTag,
}

impl OperatorBlock {
    fn new(start: OperatorTag, child: Vec<ChildStateType>, end: OperatorTag) -> Self {
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
    Branch(OperatorBlock, Vec<String>),
    /// 流程控制命令
    Command(OperatorTag, Vec<String>),
}

impl Operator {
    pub fn new_branch(block: OperatorBlock, scope: Vec<&str>) -> Self {
        Operator::Branch(block, scope.iter().map(|e| e.to_string()).collect())
    }
    pub fn new_command(tag: OperatorTag, scope: Vec<&str>) -> Self {
        Operator::Command(tag, scope.iter().map(|e| e.to_string()).collect())
    }
    pub(crate) fn get_start_tag(&self) -> &str {
        match self {
            Operator::Branch(tag, _) => {
                &tag.start.tag
            }
            Operator::Command(tag, _) => {
                &tag.tag
            }
        }
    }
}


pub fn default_state() -> Vec<Operator> {
    let mut result = Vec::new();
    result.push(Operator::new_branch(OperatorBlock::new(
        OperatorTag::new("for",
                         vec![
                             ("default", vec![ParamSyntax::Assignment,
                                              ParamSyntax::Keywords("in".to_string()),
                                              ParamSyntax::Expression])]),
        vec![],
        OperatorTag::new("end-for", vec![]),
    ), vec![]));
    result.push(Operator::new_branch(OperatorBlock::new(
        OperatorTag::new("switch",
                         vec![
                             ("default", vec![ParamSyntax::Expression])]),
        vec![ChildStateType::Multiple(OperatorTag::new("case",
                                                       vec![
                                                           ("default", vec![ParamSyntax::StaticValue])])),
             ChildStateType::Single(OperatorTag::new("default", vec![]))],
        OperatorTag::new("end-switch", vec![]),
    ), vec![]));
    //if
    result.push(Operator::new_branch(OperatorBlock::new(
        OperatorTag::new("if",
                         vec![
                             ("default", vec![ParamSyntax::Expression]),
                             ("let", vec![ParamSyntax::Assignment,
                                          ParamSyntax::Keywords("=".to_string()),
                                          ParamSyntax::Expression])]),
        vec![ChildStateType::Multiple(OperatorTag::new("else-if",
                                                       vec![
                                                           ("default", vec![ParamSyntax::Expression]),
                                                           ("let", vec![ParamSyntax::Assignment,
                                                                        ParamSyntax::Keywords("=".to_string()),
                                                                        ParamSyntax::Expression])])),
             ChildStateType::Single(OperatorTag::new("else", vec![]))],
        OperatorTag::new("end-if", vec![]),
    ), vec![]));
    result.push(Operator::new_command(OperatorTag::new("include",
                                                       vec![("default", vec![ParamSyntax::Expression])]), vec![]));
    result.push(Operator::new_command(OperatorTag::new("let",
                                                       vec![("default", vec![
                                                           ParamSyntax::Assignment,
                                                           ParamSyntax::Keywords("=".to_string()),
                                                           ParamSyntax::Expression])]), vec![]));
    result.push(Operator::new_command(OperatorTag::new("break", vec![]), vec!["loop", "for"]));
    result.push(Operator::new_command(OperatorTag::new("continue", vec![]), vec!["loop", "for"]));
    result
}
