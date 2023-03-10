use crate::statement::statement::StatementAstTree;

pub struct StateStack {
    pub root: Vec<StatementAstTree>,
    pub child_stack: Vec<StatementAstTree>,
}



impl Default for StateStack {
    fn default() -> Self {
        StateStack {
            root: vec![],
            child_stack: vec![],
        }
    }
}
