use std::collections::HashMap;

use crate::expr::common::{ExpressionAST, SymbolType};
use crate::expr::ExpressionIR;
use crate::expr::ExpressionIR::*;
use crate::expr::ExpressionManager;
use crate::template::default_expressions_symbol;

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
            ItemVariable(va) => format!(
                "${{{}}}",
                va.iter()
                    .map(|e| e.as_str())
                    .collect::<Vec<&str>>()
                    .join(".")
                    .to_string()
            ),

            ItemPrimitive(name, child) => {
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

impl Default for ExpressionManager {
    fn default() -> Self {
        ExpressionManager {
            symbols: default_expressions_symbol(),
            primitive_renders: HashMap::new(),
        }
    }
}
