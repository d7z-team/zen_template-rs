use crate::error::TemplateError::SyntaxError;
use crate::error::TmplResult;
use crate::expression::{ExpressionAST, ExpressionAstTree, ExpressionIR};

use crate::value::TemplateValue;

impl ExpressionIR {
    ///将表达式转换为 AST
    pub fn to_ast(self: Self) -> TmplResult<ExpressionAstTree> {
        let ast = match self {
            ExpressionIR::ItemGroup(_) | ExpressionIR::ItemSymbol(_) => Err(SyntaxError(format!(
                "无法将 IR 的 Group/Symbol 转换为 AST，可能是 IR 存在未知符号或语法错误."
            )))?,

            ExpressionIR::ItemValue(value) => ExpressionAstTree::ItemValue(value),
            ExpressionIR::ItemPrimitive(name, child) => ExpressionAstTree::ItemPrimitive(
                name,
                child
                    .into_iter()
                    .flat_map(|e| ExpressionIR::to_ast(e))
                    .collect::<Vec<ExpressionAstTree>>(),
            ),
            ExpressionIR::ItemVariable(mut vars) => {
                let var = vars.remove(0);
                let mut left = ExpressionAstTree::ItemVariable(var);
                for item in vars.into_iter() {
                    let right = match TemplateValue::from(&item) {
                        TemplateValue::Number(number) => TemplateValue::Number(number),
                        _ => TemplateValue::Text(item.to_string()),
                    };
                    left = ExpressionAstTree::ItemPrimitive("get".to_string(), vec![left, ExpressionAstTree::ItemValue(right)])
                }
                left
            }
        };
        Ok(ast)
    }
}

impl ExpressionAST {
    fn put_variables(ast: &Vec<ExpressionAstTree>, variables: &mut Vec<String>) {
        ast.iter().for_each(|e| match e {
            ExpressionAstTree::ItemVariable(name) => variables.push(name.to_string()),
            ExpressionAstTree::ItemPrimitive(_, child) => Self::put_variables(child, variables),
            _ => {}
        });
    }

    ///解析 AST Tree
    pub fn from(ast: ExpressionAstTree) -> Self {
        let mut variables: Vec<String> = vec![];
        let mut ast_list = vec![ast];
        Self::put_variables(&ast_list, &mut variables);
        ExpressionAST {
            variables,
            ast: ast_list.remove(0),
        }
    }
}

impl ToString for ExpressionAstTree {
    fn to_string(&self) -> String {
        match self {
            ExpressionAstTree::ItemValue(st) => {
                format!("'{}'", st.to_string())
            }
            ExpressionAstTree::ItemVariable(va) => format!("${{{}}}", va),

            ExpressionAstTree::ItemPrimitive(name, child) => {
                format!(
                    "{}({})",
                    name,
                    child
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}

impl ToString for ExpressionAST {
    fn to_string(&self) -> String {
        self.ast.to_string()
    }
}
