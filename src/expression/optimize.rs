use crate::error::TmplResult;
use crate::expression::{ExpressionAST, ExpressionAstTree, ExpressionManager, PrimitiveRenderType};
use crate::value::TemplateValue;

impl ExpressionManager {
    //优化结构
    pub fn optimize(&self, expr: ExpressionAST) -> TmplResult<ExpressionAST> {
        Ok(ExpressionAST::from(self.runner(&expr.ast, &|_| None)?))
    }
    /// 渲染变量
    pub fn runner<F>(&self, ast: &ExpressionAstTree, variable_getter: &F) -> TmplResult<ExpressionAstTree>
        where F: Fn(&str) -> Option<TemplateValue> {
        match ast {
            ExpressionAstTree::ItemValue(value) => Ok(ExpressionAstTree::ItemValue(value.clone())),
            ExpressionAstTree::ItemVariable(name) => {
                Ok(variable_getter(name.as_str()).map(|e| ExpressionAstTree::ItemValue(e.clone()))
                    .or(Some(ExpressionAstTree::ItemVariable(name.to_owned()))).unwrap())
            }
            ExpressionAstTree::ItemPrimitive(name, child) => {
                let child = child.iter().map(|e| self.runner(e, variable_getter))
                    .collect::<Vec<TmplResult<ExpressionAstTree>>>().into_iter().collect::<TmplResult<Vec<ExpressionAstTree>>>()?;
                let mut params = vec![];
                let mut fail_params = vec![];
                for new_child in child {
                    match &new_child {
                        ExpressionAstTree::ItemValue(data) => {
                            fail_params.push(new_child.clone());
                            params.push(data.clone())
                        }
                        ExpressionAstTree::ItemVariable(_) => {
                            fail_params.push(new_child);
                        }
                        ExpressionAstTree::ItemPrimitive(_, _) => {
                            fail_params.push(new_child);
                        }
                    }
                }
                return if fail_params.len() != params.len() {
                    // 存在未解析完成的
                    Ok(ExpressionAstTree::ItemPrimitive(name.clone(), fail_params))
                } else {
                    if let Some(PrimitiveRenderType::Native(primitive)) = self.primitive_renders.get(name.as_str()) {
                        Ok(ExpressionAstTree::ItemValue(primitive(params)?))
                    } else {
                        return Ok(ExpressionAstTree::ItemPrimitive(name.clone(), fail_params));
                    }
                };
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::expression::ExpressionManager;
    use crate::value::TemplateValue;


    #[test]
    fn test() {
        let manager = ExpressionManager::default();
        let mut vars = HashMap::new();
        vars.insert("first".to_string(), TemplateValue::Number(12));
        vars.insert("second".to_string(), TemplateValue::Number(12));
        let func = |src: &str| -> Option<TemplateValue> {
            vars.get(src).map(|e| e.clone())
        };
        assert_eq!(manager.runner(&manager.compile("1 + (2 * 3) + 21").unwrap()
            .ast, &func,
        ).unwrap().to_string(),"'28'".to_string());
        assert_eq!(manager.runner(&manager.compile("(var1 + 12) * (14 + 16)").unwrap()
            .ast, &func,
        ).unwrap().to_string(),"multi(add(${var1},'12'),'30')".to_string());
        assert_eq!(manager.runner(&manager.compile("(18 + 12) * (14 + 16)").unwrap()
            .ast, &func,
        ).unwrap().to_string(),"'900'".to_string())
    }
}
