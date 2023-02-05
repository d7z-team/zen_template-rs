use crate::err::TemplateError::SyntaxError;
use std::collections::HashMap;
use std::ops::Not;

use crate::err::TmplResult;
use crate::expr::DataTag::{ItemPrimitive, ItemSymbol, ItemValue, ItemVariable};
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
    pub covert: fn(DataTag, DataTag) -> DataTag,
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

#[derive(Debug, Clone, PartialEq)]
pub enum DataTag {
    //标记为符号
    ItemSymbol(String),
    ///标记为最终值
    ItemValue(TmplValue),
    ///变量
    ItemVariable(Vec<String>),
    ///原语
    ItemPrimitive(String, Vec<DataTag>),
}

// TODO: 完成表达式计算算法
// TODO: 查询括号确定是原语还是优先级配置
// TODO: 剩下的Original应该全是取变量
impl ExpressionManager {
    fn compile(&self, expr_str: &str) -> TmplResult<Expression> {
        let src = self.parse_symbols(Self::parse_str(expr_str));
        let mut src = Self::parse_group(src); // 提取表达式的原始字符串
        src = Self::covert_primitive(src);
        src = self.covert_symbol(src)?;
        println!("{}", expr_str);
        println!("{}", ItemPrimitive("root".to_string(), src).to_string());
        Ok(Expression::ItemStatic(TmplValue::Float(1.2)))
    }

    ///翻译表达式
    fn covert_symbol(&self, src: Vec<DataTag>) -> TmplResult<Vec<DataTag>> {
        let mut result = src;
        for symbol in &self.symbols {
            result = Self::covert_symbol_once(symbol, result)?
        }
        Ok(result)
    }
    fn covert_symbol_once(
        covert: &ExprSymbolCovert,
        mut src: Vec<DataTag>,
    ) -> TmplResult<Vec<DataTag>> {
        let symbol = ItemSymbol(covert.symbol.to_string());
        src = src
            .into_iter()
            .map(|e| {
                if let ItemPrimitive(name, child) = e {
                    Self::covert_symbol_once(covert, child).map(|e| ItemPrimitive(name, e))
                } else {
                    Ok(e)
                }
            })
            .collect::<TmplResult<Vec<DataTag>>>()?;
        loop {
            if let Some((k, v)) = src.iter().enumerate().find(|e| e.1.eq(&symbol)) {
                if k % 2 != 1 && k == src.len() {
                    return Err(SyntaxError(format!("此符号'{:?}'位置错误！", v)));
                }
                let right = src.remove(k + 1);
                let left = src.remove(k - 1);
                let func = &covert.covert;
                src[k - 1] = func(left, right); // 填充旧位置
            } else {
                break;
            }
        }
        return Ok(src);
    }
    ///翻译原语
    fn covert_primitive(src: Vec<DataTag>) -> Vec<DataTag> {
        let mut iter = src.into_iter();
        let mut _current = iter.next();
        let mut result = vec![];

        loop {
            let mut current = _current.unwrap();
            let mut next = iter.next();

            if let ItemPrimitive(name, child) = current {
                current = ItemPrimitive(name, Self::covert_primitive(child));
            }
            if let ItemSymbol(item) = current {
                result.push(ItemSymbol(item));
            } else {
                if let Some(ItemPrimitive(name, mut child)) = next {
                    child.insert(0, current);
                    result.push(ItemPrimitive(name, child));
                    next = iter.next();
                } else {
                    result.push(current);
                }
            }
            _current = next;
            if _current.is_none() {
                break;
            }
        }
        result
    }
    ///解析括号是否为分组或者原语
    fn parse_group(src: Vec<DataTag>) -> Vec<DataTag> {
        let mut iter = src.iter();
        let mut _current = iter.next();
        let mut result = vec![];
        let mut stack: Vec<(String, Vec<DataTag>)> = vec![];
        if Some(&ItemSymbol("(".to_string())) == _current {
            stack.push(("group".to_string(), vec![]));
            _current = iter.next();
        }
        loop {
            let current = _current.unwrap();
            let mut next = iter.next();

            let mut push_other = |data: DataTag| {
                if stack.is_empty() {
                    result.push(data)
                } else {
                    stack.last_mut().unwrap().1.push(data)
                }
            };

            if &ItemSymbol(")".to_string()) == current {
                if stack.is_empty() {
                    panic!()
                } else {
                    let last = stack.remove(stack.len() - 1);
                    let tag = ItemPrimitive(last.0, last.1);
                    if stack.is_empty() {
                        result.push(tag)
                    } else {
                        stack.last_mut().unwrap().1.push(tag)
                    }
                } //关闭字符
            } else if Some(&ItemSymbol("(".to_string())) == next {
                if let ItemSymbol(syn) = current {
                    //符号则表明为group
                    push_other(ItemSymbol(syn.to_string())); //插入当前数据
                    stack.push(("group".to_string(), vec![])); //新建
                } else if let ItemVariable(var) = current {
                    // 否则为原语
                    let mut new_var = Clone::clone(var);
                    let key = new_var.remove(new_var.len() - 1);
                    if new_var.is_empty().not() {
                        if new_var.len() == 1 {
                            push_other(match TmplValue::from(&new_var[0]) {
                                //处理参数作为数字的情况
                                TmplValue::Number(n) => ItemValue(TmplValue::Number(n)),
                                _ => ItemVariable(new_var),
                            })
                        } else {
                            push_other(ItemVariable(new_var))
                        }
                    }
                    stack.push((key, vec![]));
                } else {
                    panic!()
                }
                next = iter.next(); //跳过括号
            } else {
                push_other(Clone::clone(current))
            }
            if next.is_none() {
                break;
            }
            _current = next
        }
        result
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
                            child_content.push(Tag(ItemSymbol(symbol.to_string())));
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
                    TmplValue::Float(f) => ItemValue(TmplValue::Float(f)),
                    TmplValue::Number(n) => ItemValue(TmplValue::Number(n)),
                    TmplValue::Bool(b) => ItemValue(TmplValue::Bool(b)),
                    _ => ItemVariable(
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
                ItemVariable(v) => v.is_empty().not(),
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
                        Block::Dynamic(dy) => Tag(ItemValue(TmplValue::Text(dy.to_string()))),
                        Block::Static(st) => Original(st),
                    })
                    .collect(),
                Block::Dynamic(s) => vec![Tag(ItemValue(TmplValue::Text(s.to_string())))],
            })
            .collect::<Vec<ExprCompileData>>()
    }
}
impl ToString for DataTag {
    fn to_string(&self) -> String {
        match self {
            ItemSymbol(sy) => {
                format!(" `{}` ", sy)
            }
            ItemValue(st) => {
                format!("'{}'", st.to_string())
            }
            ItemVariable(va) => va
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<&str>>()
                .join(".")
                .to_string(),

            ItemPrimitive(name, child) => {
                format!(
                    "#{}({})",
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
            .compile(r#"(kotlin.lang.get('name') ?: kotlin.name ?: name ?: '没有').to_int() + 12.to_str() + 21.32 "#);

        // manager.compile(r#"1 + (2 * 3) / 4 == 12.to_str()"#);
    }
}
