use crate::err::TemplateError::SyntaxError;
use crate::err::TmplResult;
use crate::expr::ExpressionIR::{ItemGroup, ItemPrimitive, ItemSymbol};
use crate::expr::SymbolType::Custom;
use crate::expr::{ExprSymbolCovert, ExpressionIR, ExpressionManager};

impl ExpressionManager {
    ///翻译表达式
    pub(crate) fn covert_symbol(&self, src: Vec<ExpressionIR>) -> TmplResult<Vec<ExpressionIR>> {
        let mut result = src;
        for symbol in &self.symbols {
            result = Self::covert_symbol_once(symbol, result)?
        }
        Ok(result)
    }
    fn covert_symbol_once(
        covert: &ExprSymbolCovert,
        mut src: Vec<ExpressionIR>,
    ) -> TmplResult<Vec<ExpressionIR>> {
        let symbol = ItemSymbol(Custom(covert.symbol.to_string()));
        src = src
            .into_iter()
            .map(|e| {
                //递归处理符号转换
                if let ItemPrimitive(name, child) = e {
                    Self::covert_symbol_once(covert, child).map(|e| ItemPrimitive(name, e))
                } else if let ItemGroup(child) = e {
                    Self::covert_symbol_once(covert, child).map(|e| ItemGroup(e))
                } else {
                    Ok(e)
                }
            })
            .collect::<TmplResult<Vec<ExpressionIR>>>()?;
        while let Some((k, v)) = src.iter().enumerate().find(|e| e.1.eq(&symbol)) {
            if k % 2 != 1 && k == src.len() {
                return Err(SyntaxError(format!("此符号'{:?}'位置错误！", v)));
            }
            let right = src.remove(k + 1);
            let left = src.remove(k - 1);
            let func = &covert.covert;
            src[k - 1] = func(left, right); // 填充旧位置
        }

        return Ok(src);
    }
}
