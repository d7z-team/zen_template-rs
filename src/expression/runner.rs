use crate::error::TemplateError::{ExistsError};
use crate::error::TmplResult;
use crate::expression::ExpressionAST;
use crate::value::TemplateValue;

impl ExpressionAST {
    fn run(&self,
           primitive_func: fn(&str) -> Option<fn(Vec<TemplateValue>) -> TmplResult<TemplateValue>>,
           variable_func: fn(&str) -> TemplateValue) -> TmplResult<ExpressionAST> {
        match self {
            ExpressionAST::ItemValue(value) => { Ok(ExpressionAST::ItemValue(Clone::clone(value))) }
            ExpressionAST::ItemVariable(value) => { Ok(ExpressionAST::ItemVariable(value.to_string())) }
            ExpressionAST::ItemPrimitive(name, child) => {
                let result = child.iter().map(|e|
                    e.run(primitive_func, variable_func)).collect::<Vec<TmplResult<ExpressionAST>>>()
                    .into_iter().collect::<TmplResult<Vec<ExpressionAST>>>()?;
                let mut params = vec![];
                for item in &result {
                    match item {
                        ExpressionAST::ItemValue(value) => { params.push(value.to_owned()) }
                        ExpressionAST::ItemVariable(name) => { params.push(variable_func(name.as_str())) }
                        ExpressionAST::ItemPrimitive(name, child) => {
                            return Ok(ExpressionAST::ItemPrimitive(name.to_owned(), child.to_owned()));
                        }
                    }
                }
                if let Some(primitive) = primitive_func(name) {
                    primitive(params).map(|e| ExpressionAST::ItemValue(e))
                } else {
                    Err(ExistsError(format!("{}", name)))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::expression::{ExpressionAST, ExpressionManager};
    use crate::value::TemplateValue;

    #[test]
    fn test() {
        let manager = ExpressionManager::default();
        let expression = manager.compile("1 + 2 +3").unwrap();
        println!("{}", expression.to_string());
        println!("{:?}", expression.ast.run(|name| None, |item| TemplateValue::None));
    }
}
