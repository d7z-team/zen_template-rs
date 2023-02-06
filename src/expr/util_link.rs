use crate::err::TemplateError::SyntaxError;
use crate::err::TmplResult;
use crate::expr::common::SymbolType::Custom;
use crate::expr::common::{ExprSymbolCovert, ExpressionIR, ExpressionManager, PrimitiveRenderType};
use crate::expr::ExpressionIR::{ItemGroup, ItemPrimitive, ItemSymbol};

impl ExpressionManager {
    ///处理所有符号
    pub fn link_symbols(&self, src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        for symbol in &self.symbols {
            Self::link_symbol(symbol, src)?
        }
        Ok(())
    }
    ///处理单个符号
    fn link_symbol(covert: &ExprSymbolCovert, src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        let symbol = ItemSymbol(Custom(covert.symbol.to_string()));
        for e in src.iter_mut() {
            //递归处理符号转换
            if let ItemPrimitive(_, child) = e {
                Self::link_symbol(covert, child)?;
            } else if let ItemGroup(child) = e {
                Self::link_symbol(covert, child)?;
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

    ///转换预定义的原语
    pub fn link_static_primitives(&self, src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        for (key, func) in &self.primitive_renders {
            if let PrimitiveRenderType::Translate(func) = func {
                Self::link_static_primitive(src, key, func)?;
            }
        }
        Ok(())
    }
    fn link_static_primitive(
        src: &mut Vec<ExpressionIR>,
        name: &str,
        func: &fn(Vec<ExpressionIR>) -> TmplResult<ExpressionIR>,
    ) -> TmplResult<()> {
        for item in src.iter_mut() {
            if let ItemPrimitive(current_name, child) = item {
                if current_name == name {
                    *item = func(child.to_owned())?
                }
            }
            match item {
                ItemPrimitive(_, child) | ItemGroup(child) => {
                    Self::link_static_primitive(child, name, func)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
