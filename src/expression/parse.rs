use crate::error::TmplResult;
use crate::expression::stack::ExpressionStack;
use crate::expression::{ExpressionIR, SymbolType};
use crate::value::TemplateValue;
use crate::value::TemplateValue::Number;

impl ExpressionIR {
    ///解析括号是否为分组或者原语
    pub fn parse_groups(src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        // 转换所有开始符号
        while let Some(index) = src
            .iter()
            .enumerate()
            .find(|(_, data)| **data == ExpressionIR::ItemSymbol(SymbolType::BlockStart))
            .map(|e| e.0)
        {
            if let Some(ExpressionIR::ItemVariable(vars)) = Some(index)
                .filter(|e| *e > 0)
                .map(|i| src.get_mut(i - 1))
                .flatten()
            {
                let last = vars.remove(vars.len() - 1);
                // 确定为原语
                src[index] = ExpressionIR::ItemPrimitive(last, vec![]);
            } else {
                // 确定为包裹
                src[index] = ExpressionIR::ItemGroup(vec![]);
            }
        }
        let mut stack = ExpressionStack::default();
        while src.len() != 0 {
            let item = src.remove(0);
            match &item {
                ExpressionIR::ItemSymbol(SymbolType::BlockEnd) => {
                    if let Some(ExpressionIR::ItemPrimitive(_, _)) = stack.depth(-1) {
                        if let Some(ExpressionIR::ItemGroup(_)) = stack.depth(0) {
                            stack.end_child()?;
                        }
                    }
                    stack.end_child()?;
                }
                ExpressionIR::ItemSymbol(SymbolType::BlockCut) => {
                    stack.end_child()?;
                    stack.new_child(ExpressionIR::ItemGroup(vec![]))
                }
                ExpressionIR::ItemVariable(var) => {
                    if var.len() == 1 {
                        match TemplateValue::from(var[0].as_str()) {
                            Number(num) => stack.push(ExpressionIR::ItemValue(Number(num))),
                            _ => stack.push(item),
                        }?;
                    } else if var.len() > 1 {
                        stack.push(item)?
                    }
                }
                ExpressionIR::ItemSymbol(_) | ExpressionIR::ItemValue(_) => stack.push(item)?,
                ExpressionIR::ItemPrimitive(_, _) => {
                    stack.new_child(item);
                    stack.new_child(ExpressionIR::ItemGroup(vec![]))
                }
                ExpressionIR::ItemGroup(_) => {
                    stack.new_child(item);
                }
            }
        }
        *src = stack.close();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::expression::expression_specs::SymbolType::Custom;
    use crate::expression::tag::test::new_ir;
    use crate::expression::ExpressionIR::{ItemGroup, ItemSymbol, ItemValue};
    use crate::expression::{ExpressionIR, ExpressionManager};
    use crate::value::TemplateValue;

    #[test]
    fn test_parse_groups() {
        let manager = ExpressionManager::default();
        let covert = |src: &str| -> Vec<ExpressionIR> {
            let mut vec = manager.tagged_symbols(new_ir(src)).unwrap();
            ExpressionIR::parse_groups(&mut vec).unwrap();
            vec
        };
        assert_eq!(
            covert("1 + 2 == 3"),
            vec![
                ItemValue(TemplateValue::Number(1)),
                ItemSymbol(Custom("+".to_string())),
                ItemValue(TemplateValue::Number(2)),
                ItemSymbol(Custom("==".to_string())),
                ItemValue(TemplateValue::Number(3)),
            ]
        );
        assert_eq!(
            covert("1 && (2 == 3)"),
            vec![
                ItemValue(TemplateValue::Number(1)),
                ItemSymbol(Custom("&&".to_string())),
                ItemGroup(vec![
                    ItemValue(TemplateValue::Number(2)),
                    ItemSymbol(Custom("==".to_string())),
                    ItemValue(TemplateValue::Number(3)),
                ]),
            ]
        )
    }
}
