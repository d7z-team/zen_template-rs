use std::collections::HashMap;
use std::fmt::format;
use std::ops::Not;

use crate::err::TmplResult;
use crate::expr::DataTag::Symbol;
use crate::expr::ExprCompileData::{Fixed, Original, Tag};
use crate::template::default_expressions_symbol;
use crate::utils::str::{find, Block};
use crate::value::TmplValue;

///表达式处理器
pub struct ExpressionManager {
    ///符号表，包含符号转原语方式
    symbols: Vec<ExprSymbolCovert>,
    /// 原语
    primitive_renders: HashMap<String, PrimitiveRenderType>,
}

/// 符号转换
pub struct ExprSymbolCovert {
    ///符号
    pub symbol: String,
    /// 原语翻译函数
    pub covert: fn(Expression, Expression) -> Primitive,
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

#[derive(Debug)]
enum ExprCompileData<'a> {
    //确定的表达式
    Fixed(Expression),
    //带标记的处理数据
    Tag(&'a str, DataTag),
    //不带标记的原始数据
    Original(&'a str),
}

#[derive(Debug)]
enum DataTag {
    Symbol,
}

// TODO: 完成表达式计算算法
// TODO: 查询括号确定是原语还是优先级配置
// TODO: 剩下的Original应该全是取变量
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<Expression> {
        let mut src = Self::parse_str(expr_str);
        src = self.parse_symbols(src);
        println!(
            "{}",
            src.iter()
                .map(|e| {
                    match e {
                        Fixed(e) => format!("\'{:?}\'", e),
                        Tag(e, _) => e.to_string(),
                        Original(e) => format!("\"{}\"", e),
                    }
                })
                .collect::<String>()
        );
        todo!()
    }
    /// 替换单个字符
    fn parse_symbols_once<'a, 'b: 'a>(
        input: &mut Vec<ExprCompileData<'a>>,
        symbol: &'b str,
    ) -> Vec<ExprCompileData<'a>> {
        let mut content = vec![];
        loop {
            if input.len() > 0 {
                let data = input.remove(0);
                if let Original(src) = data {
                    let mut last_start = 0;
                    let mut child_content: Vec<ExprCompileData> = vec![];
                    loop {
                        if let Some(index) = find(src, last_start, symbol) {
                            child_content.push(Original(&src[last_start..index]));
                            child_content.push(Tag(symbol, Symbol));
                            last_start = index + symbol.len();
                        } else {
                            child_content.push(Original(&src[last_start..]));
                            break;
                        }
                    }
                    content.push(child_content);
                } else {
                    content.push(vec![data]);
                }
            } else {
                break;
            }
        }
        content.into_iter().flat_map(|e| e).collect()
    }
    /// 替换所有字符
    fn parse_symbols<'a: 'b, 'b>(
        &'a self,
        src: Vec<ExprCompileData<'b>>,
    ) -> Vec<ExprCompileData<'b>> {
        let mut src = src;
        self.symbols
            .iter()
            .map(|e| e.symbol.as_str())
            .for_each(|s| src = Self::parse_symbols_once(&mut src, s));
        // src = Self::parse_symbols_once(&mut src, "."); //预定义规则
        src = Self::parse_symbols_once(&mut src, "("); //预定义规则
        src = Self::parse_symbols_once(&mut src, ")"); //预定义规则
        src.into_iter()
            .map(|e| match e {
                Original(item) => Original(item.trim()),
                _ => e,
            })
            .filter(|e| match e {
                Original(item) => item.trim().is_empty().not(),
                _ => true,
            })
            .collect()
    }
    //源码下的原始字符串提取出来
    fn parse_str(src: &str) -> Vec<ExprCompileData> {
        Block::new_group(src.trim(), "\"", "\"", &vec![("'", "'")])
            .into_iter()
            .flat_map(|e| match e {
                Block::Static(d) => Block::new_group(d, "'", "'", &vec![("\"", "\"")])
                    .into_iter()
                    .map(|e| match e {
                        Block::Dynamic(dy) => {
                            Fixed(Expression::ItemStatic(TmplValue::Text(dy.to_string())))
                        }
                        Block::Static(st) => Original(st),
                    })
                    .collect(),
                Block::Dynamic(s) => vec![Fixed(Expression::ItemStatic(TmplValue::Text(
                    s.to_string(),
                )))],
            })
            .collect::<Vec<ExprCompileData>>()
    }
}

impl Default for ExpressionManager {
    fn default() -> Self {
        ExpressionManager {
            symbols: default_expressions_symbol(),
            primitive_renders: HashMap::new(),
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
            .compile(r#"kotlin.lang .name ?: kotlin . name ?: name "#)
            .unwrap();
    }
}
