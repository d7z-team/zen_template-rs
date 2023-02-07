use crate::error::TemplateError::SyntaxError;
use crate::error::TmplResult;
use crate::expression::{Expression, ExpressionAST, ExpressionIR};

use crate::value::TemplateValue;

impl ExpressionIR {
    ///将表达式转换为 AST
    pub fn to_ast(self: Self) -> TmplResult<ExpressionAST> {
        let ast = match self {
            ExpressionIR::ItemGroup(_) | ExpressionIR::ItemSymbol(_) => Err(SyntaxError(format!(
                "无法将 IR 的 Group/Symbol 转换为 AST，可能是 IR 存在未知符号或语法错误."
            )))?,

            ExpressionIR::ItemValue(value) => ExpressionAST::ItemValue(value),
            ExpressionIR::ItemPrimitive(name, child) => ExpressionAST::ItemPrimitive(
                name,
                child
                    .into_iter()
                    .flat_map(|e| ExpressionIR::to_ast(e))
                    .collect::<Vec<ExpressionAST>>(),
            ),
            ExpressionIR::ItemVariable(mut vars) => {
                let var = vars.remove(0);
                let mut left = ExpressionAST::ItemVariable(var);
                for item in vars.into_iter() {
                    let right = match TemplateValue::from(&item) {
                        TemplateValue::Number(number) => TemplateValue::Number(number),
                        _ => TemplateValue::Text(item.to_string()),
                    };
                    left = ExpressionAST::ItemPrimitive("get".to_string(), vec![left, ExpressionAST::ItemValue(right)])
                }
                left
            }
        };
        Ok(ast)
    }
}

impl Expression {
    fn put_variables(ast: &Vec<ExpressionAST>, variables: &mut Vec<String>) {
        ast.iter().for_each(|e| match e {
            ExpressionAST::ItemVariable(name) => variables.push(name.to_string()),
            ExpressionAST::ItemPrimitive(_, child) => Self::put_variables(child, variables),
            _ => {}
        });
    }

    ///解析 AST Tree
    pub fn from(ast: ExpressionAST) -> Self {
        let mut variables: Vec<String> = vec![];
        let mut ast_list = vec![ast];
        Self::put_variables(&ast_list, &mut variables);
        Expression {
            variables,
            ast: ast_list.remove(0),
        }
    }
}

impl ToString for ExpressionAST {
    fn to_string(&self) -> String {
        match self {
            ExpressionAST::ItemValue(st) => {
                format!("'{}'", st.to_string())
            }
            ExpressionAST::ItemVariable(va) => format!("${{{}}}", va),

            ExpressionAST::ItemPrimitive(name, child) => {
                format!(
                    "{}({})",
                    name,
                    child
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        self.ast.to_string()
    }
}
