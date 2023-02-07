mod link;
mod output;
mod parse;
mod reduce;
mod expression_specs;
mod tag;
mod expression_utils;
mod stack;
mod template;
mod utils;

use crate::error::TmplResult;
pub use crate::expression::expression_specs::{
    Expression, ExpressionAST, ExpressionIR, ExpressionManager, ExpressionSymbolCovert,
    PrimitiveRenderType, SymbolType,
};
use crate::expression::utils::ExpressCompileIR;

// TODO: 剩下语法优化
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<Expression> {
        let mut src = self.tagged_symbols(ExpressCompileIR::parse_static_str(expr_str))?;
        ExpressionIR::parse_groups(&mut src)?; // 提取表达式的原始字符串
        ExpressionIR::compile_primitives(&mut src)?; // 处理原语 (此时不应该有任何的未知符号)
        self.link_symbols(&mut src)?; // 替换符号
        ExpressionIR::compile_primitives(&mut src)?; // 处理原语 (此时不应该有任何的未知符号)
        self.link_static_primitives(&mut src)?; //渲染静态函数
        let mut ir = ExpressionIR::ItemGroup(src);
        ExpressionIR::flat_depth(&mut ir)?;
        Ok(Expression::from(ir.to_ast()?))
    }
}

#[cfg(test)]
mod test {
    use crate::expression::ExpressionManager;

    #[test]
    fn test() {
        let manager = ExpressionManager::default();
        println!(
            "{:#?}",
            manager
                .compile(r#"(1+2)*3/12-31%121 in 1212.get('121'.to_str(12))"#)
                .map_err(|e| e.to_string())
                .unwrap()
                .to_string()
        );
    }
}
