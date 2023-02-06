use crate::err::TemplateError::SyntaxError;
use crate::err::TmplResult;
use crate::expr::ExpressionIR::{ItemGroup, ItemPrimitive, ItemSymbol};
use crate::expr::SymbolType::Custom;
use crate::expr::{ExprSymbolCovert, ExpressionIR, ExpressionManager};

impl ExpressionManager {
    ///翻译表达式
    pub fn compile_symbols(&self, src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        for symbol in &self.symbols {
            Self::covert_symbol_once(symbol, src)?
        }
        Ok(())
    }
    fn covert_symbol_once(
        covert: &ExprSymbolCovert,
        src: &mut Vec<ExpressionIR>,
    ) -> TmplResult<()> {
        let symbol = ItemSymbol(Custom(covert.symbol.to_string()));
        for e in src.iter_mut() {
            //递归处理符号转换
            if let ItemPrimitive(_, child) = e {
                Self::covert_symbol_once(covert, child)?;
            } else if let ItemGroup(child) = e {
                Self::covert_symbol_once(covert, child)?;
            }
        }
        while let Some((k, v)) = src.iter().enumerate().find(|e| e.1.eq(&symbol)) {
            if k % 2 != 1 && k == src.len() {
                return Err(SyntaxError(format!("此符号'{:?}'位置错误！", v)));
            }
            let right = src.remove(k + 1);
            let left = src.remove(k - 1);
            if let ItemSymbol(_) = left {
                return Err(SyntaxError(format!(
                    "符号 {} 位置错误！ ,\n{:#?}",
                    left.to_string(),
                    src
                )));
            }
            if let ItemSymbol(_) = right {
                return Err(SyntaxError(format!(
                    "符号 {} 位置错误！ ,\n{:#?}",
                    right.to_string(),
                    src
                )));
            }
            let func = &covert.covert;
            src[k - 1] = func(left, right); // 填充旧位置
        }

        return Ok(());
    }
}
