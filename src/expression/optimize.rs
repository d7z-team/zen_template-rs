use crate::error::TmplResult;
use crate::expression::{Expression, ExpressionManager};

impl ExpressionManager {
    //优化结构
    pub fn optimize(&self, expr: Expression) -> TmplResult<Expression> {
        // self.primitive_renders
        let ast = expr.ast;
        todo!()
    }
}
