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
        ExpressionIR::covert_primitives(&mut src)?;
        println!("{:?}", src);
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
                .compile(r#"1+(2+3)+4==5.to_string(1.to_string())"#)
                .unwrap()
                .to_string()
        );
    }
}
