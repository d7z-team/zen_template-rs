/// 表达式符号映射表，将相关的表达式转换为原语
pub fn default_expressions_symbol() -> Vec<(String, String)> {
    let mut syn_map = Vec::new();
    let mut register = |tag: &str, evolution: &str| {
        syn_map.push((tag.to_string(), evolution.to_string()))
    };
    register(".", "get($1,$2)");
    register("?:", "get_or_default($1,$2)");
    register("?.", "get_or_none($1,$2)");

    register("*", "multi($1,$2)");
    register("/", "div($1,$2)");
    register("%", "mod($1,$2)");

    register("+", "add($1,$2)");
    register("-", "sub($1,$2)");

    register(" is ", "eq(type($1),$2)");
    register("==", "eq($1,$2)");
    register("!=", "not_eq($1,$2)");
    register(">=", "ge($1,$2)");
    register("<=", "le($1,$2)");
    register(">", "r_angle($1,$2)");
    register("<", "l_angle($1,$2)");

    register("&&", "and($1,$2)");
    register("||", "or($1,$2)");
    syn_map
}

/// 流程控制关键字和语法标记
pub struct OperatorTag {
    tag: String,
    syntax: Vec<String>,
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
    result.push(Operator::new_branch("if", vec!["else-if", "else"], "end-if"));
    result.push(Operator::new_command("include", vec![]));
    result.push(Operator::new_command("let", vec![]));
    result.push(Operator::new_command("break", vec!["loop", "for"]));
    result.push(Operator::new_command("continue", vec!["loop", "for"]));
    result
}
