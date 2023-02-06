use crate::expr::common::{ExprSymbolCovert, ExpressionIR};

/// 表达式符号映射表，将相关的表达式转换为原语
pub fn default_expressions_symbol() -> Vec<ExprSymbolCovert> {
    let mut result = Vec::new();
    let mut register = |tag: &str, evolution: fn(ExpressionIR, ExpressionIR) -> ExpressionIR| {
        result.push(ExprSymbolCovert {
            symbol: tag.to_string(),
            covert: evolution,
        })
    };
    register("?:", |e, a| {
        ExpressionIR::ItemPrimitive("get_or".to_string(), vec![e, a])
    });

    register("*", |e, a| {
        ExpressionIR::ItemPrimitive("multi".to_string(), vec![e, a])
    });
    register("/", |e, a| {
        ExpressionIR::ItemPrimitive("div".to_string(), vec![e, a])
    });
    register("%", |e, a| {
        ExpressionIR::ItemPrimitive("mod".to_string(), vec![e, a])
    });

    register("+", |e, a| {
        ExpressionIR::ItemPrimitive("add".to_string(), vec![e, a])
    });
    register("-", |e, a| {
        ExpressionIR::ItemPrimitive("sub".to_string(), vec![e, a])
    });

    register(" is ", |e, a| {
        ExpressionIR::ItemPrimitive(
            "eq".to_string(),
            vec![ExpressionIR::ItemPrimitive("type".to_string(), vec![e]), a],
        )
    });
    register(" in ", |e, a| {
        ExpressionIR::ItemPrimitive("in".to_string(), vec![e, a])
    });
    register("==", |e, a| {
        ExpressionIR::ItemPrimitive("eq".to_string(), vec![e, a])
    });
    register("!=", |e, a| {
        ExpressionIR::ItemPrimitive(
            "not".to_string(),
            vec![ExpressionIR::ItemPrimitive("eq".to_string(), vec![e, a])],
        )
    });
    register(">=", |e, a| {
        ExpressionIR::ItemPrimitive("ge".to_string(), vec![e, a])
    });
    register("<=", |e, a| {
        ExpressionIR::ItemPrimitive("le".to_string(), vec![e, a])
    });
    register(">", |e, a| {
        ExpressionIR::ItemPrimitive("r_angle".to_string(), vec![e, a])
    });
    register("<", |e, a| {
        ExpressionIR::ItemPrimitive("l_angle".to_string(), vec![e, a])
    });
    register("&&", |e, a| {
        ExpressionIR::ItemPrimitive("and".to_string(), vec![e, a])
    });
    register("||", |e, a| {
        ExpressionIR::ItemPrimitive("or".to_string(), vec![e, a])
    });

    result
}
