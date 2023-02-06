use crate::err::TmplResult;
use crate::expr::common::{ExpressionIR, ExpressionManager};
use crate::expr::utils::ExpressCompileIR;
use crate::expr::ExpressionIR::*;

pub mod common;
pub mod common_utils;
pub mod optimization;
pub mod stack;
pub mod util_group;
pub mod util_link;
pub mod util_reduce;
pub mod util_tag;
pub mod utils;

// TODO: 剩下语法优化
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<ExpressionIR> {
        let mut src = self.tagged_symbols(ExpressCompileIR::parse_static_str(expr_str))?;
        ExpressionIR::parse_groups(&mut src)?; // 提取表达式的原始字符串
        self.link_symbols(&mut src)?; // 替换符号
        ExpressionIR::compile_primitives(&mut src)?; // 处理原语 (此时不应该有任何的未知符号)
        self.link_static_primitives(&mut src)?; //渲染静态函数

        Ok(ItemGroup(src))
    }
}

#[cfg(test)]
mod test {
    use crate::expr::ExpressionManager;

    #[test]
    fn test() {
        let manager = ExpressionManager::default();
        println!(
            "{:#?}",
            manager
                .compile(r#"(1+2)*3"#)
                .map_err(|e| e.to_string())
                .unwrap()
                .to_string()
        );
    }
}
