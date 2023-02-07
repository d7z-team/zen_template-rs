use crate::error::TmplResult;
use crate::expression::{Expression, ExpressionIR, ExpressionManager};

use crate::expression::utils::ExpressCompileIR;

impl ExpressionManager {
    //将表达式转换为 AST
    pub fn compile(&self, expr_str: &str) -> TmplResult<Expression> {
        let mut src = self.tagged_symbols(ExpressCompileIR::parse_static_str(expr_str))?;
        ExpressionIR::parse_groups(&mut src)?; // 提取表达式的原始字符串
        ExpressionIR::compile_primitives(&mut src)?; // 处理原语 (此时不应该有任何的未知符号)
        self.link_symbols(&mut src)?; // 替换符号
        ExpressionIR::compile_primitives(&mut src)?; // 处理原语 (此时不应该有任何的未知符号)
        // println!("{:#?}", ExpressionIR::ItemGroup(src.clone()).to_string());
        self.link_static_primitives(&mut src)?; //渲染静态函数
        let mut ir = ExpressionIR::ItemGroup(src);
        ExpressionIR::flat_depth(&mut ir)?;
        self.optimize(Expression::from(ir.to_ast()?))
    }
}
