use crate::expression::ExpressionAST;

pub struct Statement {}

// 流程控制最小化单元
pub enum StatementAstTree {
    /// 变量渲染，属于控制对象
    ItemValue(StatementValue),
    /// 流程控制，属于分支对象
    ItemBranch(StateBranch),
    /// 指令控制,属于控制对象
    ItemCommand(StateCommand),
}

pub enum StatementValue {
    ItemStatic(String),
    ItemDynamic(ExpressionAST),
}

pub struct StateBranch {
    keyword: String,
    loops: bool,
    child: Vec<StateBranchProcess>,
}

///分支流程记录
pub struct StateBranchProcess {
    keyword: String,
    params: Vec<StateParamType>,
    child_states: Vec<StatementAstTree>,
}
/// 控制命令
pub struct StateCommand {
    keyword: String,
    params: Vec<StateParamType>,
}

//参数类型
pub enum StateParamType {
    // 关键字
    Keyword(String),
    //变量： abc 、abc,abd、_,abc
    Variables,
    // 表达式 （一般前后跟随）关键字，否则不便于匹配
    Expression,
    // 静态数据
    StaticValue,
}
