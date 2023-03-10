use crate::error::TmplResult;
use crate::statement::statement::StatementAstTree;

/// 流程控制台构建
pub struct StateBuilder {
    pub root: Vec<StatementAstTree>,
    pub child_stack: Vec<StatementAstTree>,
}

impl StateBuilder {
    pub fn stack(&self)->Option<&StatementAstTree> {
        todo!()
    }
}

impl StateBuilder {
    pub fn add(&mut self, node: StatementAstTree) -> TmplResult<()> {

        todo!()
    }
}


impl Default for StateBuilder {
    fn default() -> Self {
        StateBuilder {
            root: vec![],
            child_stack: vec![],
        }
    }
}
