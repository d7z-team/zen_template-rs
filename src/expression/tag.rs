use std::ops::Not;

use crate::error::TmplResult;
use crate::expression::ExpressCompileIR::*;
use crate::expression::ExpressionIR::*;
use crate::expression::{ExpressCompileIR, ExpressionIR, ExpressionManager, SymbolType};
use crate::utils::StringUtils;
use crate::value::TemplateValue;

impl ExpressionManager {
    /// 将未定义的数据进行处理并标记
    fn tagged_symbols_once(
        input: &mut Vec<ExpressCompileIR>,
        symbol: SymbolType,
    ) -> TmplResult<()> {
        let mut content = vec![];
        loop {
            if input.len() > 0 {
                let data = input.remove(0);
                if let Original(src) = data {
                    let mut last_start = 0;
                    let mut child_content: Vec<ExpressCompileIR> = vec![];
                    loop {
                        if let Some(index) = StringUtils::find(src, last_start, &symbol.to_string())
                        {
                            if let Some(data) =
                                Some(&src[last_start..index]).filter(|e| e.is_empty().not())
                            {
                                child_content.push(Original(data));
                            }
                            child_content.push(Tag(ItemSymbol(symbol.clone())));
                            last_start = index + &symbol.to_string().len();
                        } else {
                            if let Some(data) =
                                Some(&src[last_start..]).filter(|e| e.is_empty().not())
                            {
                                child_content.push(Original(data));
                            }
                            break;
                        }
                    }
                    content.push(child_content);
                } else {
                    content.push(vec![data]);
                }
            } else {
                break;
            }
        }
        *input = content.into_iter().flat_map(|e| e).collect();
        Ok(())
    }
    /// 标记所有字符
    pub fn tagged_symbols<'a: 'b, 'b>(
        &'a self,
        src: Vec<ExpressCompileIR<'b>>,
    ) -> TmplResult<Vec<ExpressionIR>> {
        let mut src = src;
        for item in &self.symbols {
            Self::tagged_symbols_once(&mut src, SymbolType::Custom(item.symbol.to_string()))?;
        }
        Self::tagged_symbols_once(&mut src, SymbolType::BlockStart)?; //预定义规则
        Self::tagged_symbols_once(&mut src, SymbolType::BlockEnd)?; //预定义规则
        Self::tagged_symbols_once(&mut src, SymbolType::BlockCut)?; //预定义规则
        Ok(src
            .into_iter()
            .map(|e| match e {
                Original(data) => match TemplateValue::from(data.trim()) {
                    //此时只剩下变量与静态数据
                    TemplateValue::Float(f) => ItemValue(TemplateValue::Float(f)),
                    TemplateValue::Number(n) => ItemValue(TemplateValue::Number(n)),
                    TemplateValue::Bool(b) => ItemValue(TemplateValue::Bool(b)),
                    _ => ItemVariable(
                        data.trim()
                            .split(".")
                            .filter(|e| e.trim().is_empty().not())
                            .map(|e| e.to_string())
                            .collect(),
                    ), //由于 str 的声明方式不同，则此处的所有内容均标记为变量
                },
                Tag(e) => e,
            })
            .filter(|e| match e {
                ItemVariable(v) => v.is_empty().not(),
                _ => true,
            })
            .collect())
    }
}

#[cfg(test)]
pub mod test {
    use crate::error::TmplResult;
    use crate::expression::expression_specs::SymbolType::Custom;
    use crate::expression::ExpressCompileIR;
    use crate::expression::ExpressCompileIR::{Original, Tag};
    use crate::expression::ExpressionIR::{ItemSymbol, ItemValue};
    use crate::expression::ExpressionManager;
    use crate::value::TemplateValue::Number;

    #[test]
    fn test_tagged_symbols() {
        let manager = ExpressionManager::default();
        assert_eq!(
            manager.tagged_symbols(new_ir("(1+1)==2")).unwrap(),
            vec![
                ItemSymbol(Custom("(".to_string())),
                ItemValue(Number(1)),
                ItemSymbol(Custom("+".to_string())),
                ItemValue(Number(1)),
                ItemSymbol(Custom(")".to_string())),
                ItemSymbol(Custom("==".to_string())),
                ItemValue(Number(2)),
            ]
        )
    }

    pub fn new_ir(src: &str) -> Vec<ExpressCompileIR> {
        ExpressCompileIR::parse_static_str(src)
    }

    fn new_result<'a>(src: &'a str, tag: &str) -> TmplResult<Vec<ExpressCompileIR<'a>>> {
        let mut vec = ExpressCompileIR::parse_static_str(src);
        ExpressionManager::tagged_symbols_once(&mut vec, Custom(tag.to_string()))?;
        Ok(vec)
    }

    fn new_symbol_tag(symbol: &str) -> ExpressCompileIR {
        Tag(ItemSymbol(Custom(symbol.to_string())))
    }

    #[test]
    fn test_tagged_symbol_once() {
        assert_eq!(
            new_result("1+2+3==4", "+").unwrap(),
            vec![
                Original("1"),
                new_symbol_tag("+"),
                Original("2"),
                new_symbol_tag("+"),
                Original("3==4"),
            ]
        );
        assert_eq!(new_result("10", "+").unwrap(), vec![Original("10")]);
        assert_eq!(new_result("10++10", "-").unwrap(), vec![Original("10++10")]);
    }
}
