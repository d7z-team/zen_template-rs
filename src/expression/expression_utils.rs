use crate::expression::template::{default_expressions_symbol, default_primitive_renders};
use crate::expression::ExpressionManager;
use crate::expression::{ExpressionIR, SymbolType};

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
            ExpressionIR::ItemSymbol(sy) => {
                format!(" {} ", sy.to_string())
            }
            ExpressionIR::ItemValue(st) => {
                format!("'{}'", st.to_string())
            }
            ExpressionIR::ItemVariable(va) => format!(
                "${{{}}}",
                va.iter()
                    .map(|e| e.as_str())
                    .collect::<Vec<&str>>()
                    .join(".")
            ),

            ExpressionIR::ItemPrimitive(name, child) => {
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
            ExpressionIR::ItemGroup(child) => {
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
        let (symbols, mut primitive_renders) = default_expressions_symbol();
        for (k,v) in default_primitive_renders().into_iter() {
            primitive_renders.insert(k,v);
        };
        ExpressionManager {
            symbols,
            primitive_renders,
        }
    }
}
