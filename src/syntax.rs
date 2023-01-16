use std::collections::HashSet;

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

/// 流程控制关键字和语法标记
pub struct OperatorTag {
    // 操作符标记
    tag: String,
    // 操作符语法
    syntax: HashSet<String>,
}

/// 带子流程的操作运算
pub struct OperatorBlock {
    /// 开始关键字
    start: OperatorTag,
    /// 子分支
    child_state: Vec<OperatorTag>,
    /// 结束关键字
    end: OperatorTag,
}

/// 流程操作符分类
pub enum Operator {
    /// 流程分支
    Branch(OperatorBlock, Vec<String>),
    /// 流程控制命令
    Command(OperatorTag, Vec<String>),
}

impl Operator {
    fn new_branch(start: &str, child: Vec<&str>, end: &str) -> Self {
        Operator::Branch(
            start.to_string(),
            child.iter().map(|e| e.to_string()).collect(),
            end.to_string(),
        )
    }
    fn new_command(cmd: &str, scope: Vec<&str>) -> Self {
        Operator::Command(cmd.to_string(), scope.iter().map(|e| e.to_string()).collect())
    }
}


pub fn default_state() -> Vec<Operator> {
    let mut result = Vec::new();
    result.push(Operator::new_branch("for", vec![], "end-for"));
    result.push(Operator::new_branch("loop", vec![], "end-loop"));
    result.push(Operator::new_branch("switch", vec!["case","default"], "end-switch"));
    result.push(Operator::new_branch("if", vec!["else-if", "else"], "end-if"));
    result.push(Operator::new_command("include", vec![]));
    result.push(Operator::new_command("let", vec![]));
    result.push(Operator::new_command("break", vec!["loop", "for"]));
    result.push(Operator::new_command("continue", vec!["loop", "for"]));
    result
}
