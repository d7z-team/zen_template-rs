use crate::expr::ExpressionIR;

/// 表达式转换的原语
#[derive(Debug)]
pub struct Primitive {
    ///原语名称
    key: String,
    ///原语参数
    args: Vec<ExpressionIR>,
}

impl Primitive {
    pub fn new(name: &str, args: Vec<ExpressionIR>) -> Self {
        Primitive {
            key: name.to_string(),
            args,
        }
    }
}
