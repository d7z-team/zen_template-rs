use crate::err::TmplResult;
use crate::TemplateConfig;
use crate::value::TmplValue;

///表达式
#[derive(Debug)]
enum Expression {
    Value(TmplValue),
    Primitive(Primitive),
}

/// 原语
#[derive(Debug)]
pub(crate) struct Primitive {
    name: String,
    args: Vec<Expression>,
}

pub struct TemplateAst {}

pub struct Compile {}

impl Compile {
    pub(crate) fn build_template(_src: &str, _config: &TemplateConfig) -> TmplResult<TemplateAst> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::compile::{Expression, Primitive};
    use crate::value::TmplValue;

    #[test]
    fn test() {
        Primitive {
            name: "format".to_string(),
            args: vec![
                Expression::Primitive(Primitive {
                    name: "add".to_string(),
                    args: vec![],
                }),
                Expression::Value(TmplValue::Text("utc".to_string())),
            ],
        };
    }
}
