pub use express::*;
pub use groups::*;
pub use manager::*;
pub use primitive::*;
pub use primitives::*;
pub use stack::*;
pub use symbols::*;
pub use utils::*;

use crate::err::TmplResult;
use crate::expr::ExpressionIR::*;

pub mod express;
pub mod groups;
pub mod manager;
pub mod primitive;
pub mod primitives;
pub mod stack;
pub mod symbol;
pub mod symbols;
pub mod utils;

// TODO: 剩下语法优化
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<ExpressionIR> {
        let mut src = self.tagged_symbols(ExpressCompileIR::parse_static_str(expr_str))?;
        ExpressionIR::parse_groups(&mut src)?; // 提取表达式的原始字符串
        self.compile_symbols(&mut src)?; //
        ExpressionIR::compile_primitives(&mut src)?;
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
            manager.compile(r#"(1++2)*3"#).map_err(|e| e.to_string())
        );
    }
}
