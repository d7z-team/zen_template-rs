use std::collections::HashMap;
use std::ops::Not;

use crate::err::TmplResult;
use crate::expr::DataTag::{Symbol, Value, Variable};
use crate::expr::ExprCompileData::{Original, Tag};
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
    ItemDynamic(DynamicType),
}

/// 动态表达式类型
#[derive(Debug)]
pub enum DynamicType {
    ///上下文变量
    Variable(String),
    ///原语
    Primitive(Primitive),
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
    //带标记的处理数据
    Tag(DataTag),
    //不带标记的原始数据
    Original(&'a str),
}

#[derive(Debug, Clone)]
enum DataTag {
    //标记为符号
    Symbol(String),
    ///标记为最终值
    Value(TmplValue),
    ///变量
    Variable(Vec<String>),
    ///原语
    Primitive(String, Vec<DataTag>),
}

// TODO: 完成表达式计算算法
// TODO: 查询括号确定是原语还是优先级配置
// TODO: 剩下的Original应该全是取变量
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<Expression> {
        let mut src = self.parse_symbols(Self::parse_str(expr_str)); // 提取表达式的原始字符串
        for (i, v) in src.iter().enumerate() {
            if let Symbol(value) = v {
                //校验符号合法性
                let next = src.get(i + 1);
                if *value == "(" {
                    //符号开始
                    if let Some(Symbol(item)) = next {
                        //左方为符号则表明这是一个优先级定义符号，
                    } else if let Some(Variable(var)) = next {
                        //左方为变量则表明这是一个原语的的一部分
                    } else {
                    }
                }
            }
        }
        println!("{:#?}", src);
        println!(
            "{}",
            src.iter()
                .map(|e| match e {
                    Symbol(sy) => {
                        format!(" {} ", sy)
                    }
                    Value(st) => {
                        format!("'{}'", st.to_string())
                    }
                    Variable(va) => {
                        va.iter()
                            .map(|e| e.as_str())
                            .collect::<Vec<&str>>()
                            .join(".")
                            .to_string()
                    }
                    _ => "".to_string(),
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
                            child_content.push(Tag(Symbol(symbol.to_string())));
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
    fn parse_symbols<'a: 'b, 'b>(&'a self, src: Vec<ExprCompileData<'b>>) -> Vec<DataTag> {
        let mut src = src;
        self.symbols
            .iter()
            .map(|e| e.symbol.as_str())
            .for_each(|s| src = Self::parse_symbols_once(&mut src, s));
        src = Self::parse_symbols_once(&mut src, "("); //预定义规则
        src = Self::parse_symbols_once(&mut src, ")"); //预定义规则
        src.into_iter()
            .map(|e| match e {
                Original(data) => match TmplValue::from(data.trim()) {
                    //此时只剩下变量与静态数据
                    TmplValue::Float(f) => Value(TmplValue::Float(f)),
                    TmplValue::Number(n) => Value(TmplValue::Number(n)),
                    TmplValue::Bool(b) => Value(TmplValue::Bool(b)),
                    _ => Variable(
                        data.trim()
                            .split(".")
                            .filter(|e| e.trim().is_empty().not())
                            .map(|e| e.to_string())
                            .collect(),
                    ), //由于 str 的声明方式不同，则此处的所有内容均标记为变量
                },
                Tag(e) => e,
            })
            .filter(|e| match e {
                Variable(v) => v.is_empty().not(),
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
                        Block::Dynamic(dy) => Tag(Value(TmplValue::Text(dy.to_string()))),
                        Block::Static(st) => Original(st),
                    })
                    .collect(),
                Block::Dynamic(s) => vec![Tag(Value(TmplValue::Text(s.to_string())))],
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
            .compile(r#"(kotlin.lang.get('name') ?: kotlin .name ?: name ?: '没有').to_int() + 12 + 21.32 "#)
            .unwrap();
    }
}
