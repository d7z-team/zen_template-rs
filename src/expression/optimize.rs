use crate::error::TmplResult;
use crate::expression::{Expression, ExpressionAST, ExpressionManager, PrimitiveRenderType};
use crate::value::TemplateValue;

impl ExpressionManager {
    //优化结构
    pub fn optimize(&self, expr: Expression) -> TmplResult<Expression> {
        Ok(Expression::from(self.runner(&expr.ast, &|_| None)?))
    }
    /// 渲染变量
    pub fn runner<F>(&self, ast: &ExpressionAST, variable_getter: &F) -> TmplResult<ExpressionAST>
        where F: Fn(&str) -> Option<TemplateValue> {
        match ast {
            ExpressionAST::ItemValue(value) => Ok(ExpressionAST::ItemValue(value.clone())),
            ExpressionAST::ItemVariable(name) => {
                Ok(variable_getter(name.as_str()).map(|e| ExpressionAST::ItemValue(e.clone()))
                    .or(Some(ExpressionAST::ItemVariable(name.to_owned()))).unwrap())
            }
            ExpressionAST::ItemPrimitive(name, child) => {
                let child = child.iter().map(|e| self.runner(e, variable_getter))
                    .collect::<Vec<TmplResult<ExpressionAST>>>().into_iter().collect::<TmplResult<Vec<ExpressionAST>>>()?;
                let mut params = vec![];
                let mut fail_params = vec![];
                for new_child in child {
                    match &new_child {
                        ExpressionAST::ItemValue(data) => {
                            fail_params.push(new_child.clone());
                            params.push(data.clone())
                        }
                        ExpressionAST::ItemVariable(_) => {
                            fail_params.push(new_child);
                        }
                        ExpressionAST::ItemPrimitive(_, _) => {
                            fail_params.push(new_child);
                        }
                    }
                }
                return if fail_params.len() != params.len() {
                    // 存在未解析完成的
                    Ok(ExpressionAST::ItemPrimitive(name.clone(), fail_params))
                } else {
                    if let Some(PrimitiveRenderType::Native(primitive)) = self.primitive_renders.get(name.as_str()) {
                        Ok(ExpressionAST::ItemValue(primitive(params)?))
                    } else {
                        return Ok(ExpressionAST::ItemPrimitive(name.clone(), fail_params));
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
