use crate::expr::ExpressionIR::*;
use crate::value::TmplValue;

/// 表达式编译中间代码
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionIR {
    ///标记为符号
    ItemSymbol(SymbolType),
    ///标记为最终值
    ItemValue(TmplValue),
    ///变量
    ItemVariable(Vec<String>),
    ///原语（名称，参数）
    ItemPrimitive(String, Vec<ExpressionIR>),
    /// 一组表达式
    ItemGroup(Vec<ExpressionIR>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SymbolType {
    BlockStart,
    BlockEnd,
    BlockCut,
    Custom(String),
}

impl ToString for SymbolType {
    fn to_string(&self) -> String {
        match self {
            SymbolType::BlockStart => "(".to_string(),
            SymbolType::BlockEnd => ")".to_string(),
            SymbolType::BlockCut => ",".to_string(),
            SymbolType::Custom(data) => data.to_string(),
        }
    }
}

impl ToString for ExpressionIR {
    fn to_string(&self) -> String {
        match self {
            ItemSymbol(sy) => {
                format!(" `{}` ", sy.to_string())
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
                        .join(", ")
                )
            }
            ItemGroup(child) => {
                format!(
                    "({})",
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
