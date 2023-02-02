use crate::err::TmplResult;
use crate::template::default_expressions_symbol;
use crate::utils::str::Block;
use crate::value::TmplValue;

///表达式处理器
pub struct ExpressionManager {
    ///符号表，包含符号转原语方式
    symbols: Vec<ExprSymbolCovert>,
    /// 原语
    primitive_renders: Vec<PrimitiveRender>,
}

/// 符号转换
pub struct ExprSymbolCovert {
    ///符号
    pub symbol: String,
    /// 原语翻译函数
    pub covert: fn(Expression, Expression) -> Primitive,
}

///原语执行器
pub struct PrimitiveRender {
    key: String,
    render_type: PrimitiveRenderType,
}

///原语渲染方案
pub enum PrimitiveRenderType {
    ///原语渲染：对输入的数据进行计算，并返回数据
    Native(fn(Vec<TmplValue>) -> TmplResult<TmplValue>),
    ///原语翻译：原语翻译，将高级原语翻译为低级原语
    Translate(fn(Vec<Expression>) -> TmplResult<Primitive>),
}

///表达式包装
#[derive(Debug)]
pub enum Expression {
    ///静态数据，可直接输出
    ItemStatic(TmplValue),
    ///动态表达式，需要结合原语计算
    ItemDynamic(Primitive),
}

/// 表达式转换的原语
#[derive(Debug)]
pub struct Primitive {
    ///原语名称
    key: String,
    ///原语参数
    args: Vec<Expression>,
}

impl Primitive {
    pub fn new(name: &str, args: Vec<Expression>) -> Self {
        Primitive {
            key: name.to_string(),
            args,
        }
    }
}

// TODO: 完成表达式计算算法
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<Expression> {
        let vec1 = Block::new_group(expr_str, "\"", "\"", &vec![("'", "'")]);
        println!("{:?}", vec1);
        todo!()
    }
}

impl Default for ExpressionManager {
    fn default() -> Self {
        ExpressionManager {
            symbols: default_expressions_symbol(),
            primitive_renders: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::expr::ExpressionManager;

    #[test]
    fn test() {
        let manager = ExpressionManager::default();
        manager
            .compile(r#"这是一段测试的文本 "里面" 外面"#)
            .unwrap();
    }
}
