use crate::err::TemplateError::SyntaxError;
use crate::err::TmplResult;
use crate::expr::common::Expression;
use crate::expr::common::ExpressionAST;
use crate::expr::common::ExpressionIR;
use crate::value::TmplValue;

impl ExpressionIR {
    ///将表达式转换为 AST
    pub fn to_ast(self: Self) -> TmplResult<ExpressionAST> {
        let ast = match self {
            ExpressionIR::ItemGroup(_) | ExpressionIR::ItemSymbol(_) => Err(SyntaxError(format!(
                "无法将 IR 的 Group/Symbol 转换为 AST，可能是 IR 未解析完成"
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
                let mut params = vars
                    .iter()
                    .map(|e| {
                        match TmplValue::from(e.as_str()) {
                            TmplValue::Number(number) => TmplValue::Number(number),
                            _ => TmplValue::Text(e.to_string()),
                        };
                        ExpressionAST::ItemValue(TmplValue::Text("s".to_string()))
                    })
                    .collect::<Vec<ExpressionAST>>();
                params.insert(0, ExpressionAST::ItemVariable(var.to_string()));
                ExpressionAST::ItemPrimitive("get".to_string(), params)
            }
        };
        Ok(ast)
    }
}

impl Expression {
    fn get_variables(ast: &Vec<ExpressionAST>, variables: &mut Vec<String>) {
        ast.iter().for_each(|e| match e {
            ExpressionAST::ItemVariable(name) => variables.push(name.to_string()),
            ExpressionAST::ItemPrimitive(_, child) => Self::get_variables(child, variables),
            _ => {}
        });
    }

    pub fn from(ast: ExpressionAST) -> Self {
        let mut variables: Vec<String> = vec![];
        let mut ast_list = vec![ast];
        Self::get_variables(&ast_list, &mut variables);
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
