pub use covert_primitives::*;
pub use covert_symbols::*;
pub use express::*;
pub use manager::*;
pub use parse_groups::*;
pub use primitive::*;
pub use stack::*;
pub use utils::*;

use crate::err::TmplResult;
use crate::expr::ExpressionIR::*;

pub mod covert_primitives;
pub mod covert_symbols;
pub mod express;
pub mod manager;
pub mod parse_groups;
pub mod primitive;
pub mod stack;
pub mod tagged_symbols;
pub mod utils;

// TODO: 完成表达式计算算法
// TODO: 查询括号确定是原语还是优先级配置
// TODO: 剩下的Original应该全是取变量
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<ExpressionIR> {
        let mut src = self.tagged_symbols(ExpressCompileIR::parse_static_str(expr_str))?;
        ExpressionIR::parse_groups(&mut src)?; // 提取表达式的原始字符串
        src = ExpressionIR::covert_primitives(src);
        src = self.covert_symbol(src)?;
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
                .compile(r#"(kotlin.lang.get('name',data,to_str(), sa.item()) ?: kotlin.name ?: name ?: '没有').to_int() + 12.to_str() + 21.32 "#)
                .unwrap().to_string()
        );

        println!(
            "{:?}",
            manager
                .compile(r#"(1 + (2 * 3) / 4 )== 12.to_str()"#)
                .unwrap()
                .to_string()
        );
    }
}
