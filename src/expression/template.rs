mod primitive;
mod compile;
pub  use compile::*;

use primitive::*;
use std::collections::HashMap;
use ExpressionIR::ItemPrimitive;
use crate::error::TmplResult;
use crate::expression::{ExpressionIR, ExpressionSymbolCovert, PrimitiveRenderType};
use crate::expression::expression_specs::NativePrimitiveRender;
use crate::expression::PrimitiveRenderType::Native;
use crate::value::TemplateValue;

enum PrimitiveType {
    Static(String, NativePrimitiveRender),
    Empty,
}

/// 表达式符号映射表，将相关的表达式转换为原语
pub fn default_expressions_symbol() -> (Vec<ExpressionSymbolCovert>, HashMap<String, PrimitiveRenderType>) {
    let mut coverts = Vec::new();
    let mut primitives = HashMap::new();

    let mut register_symbol = |tag: &str, evolution: fn(ExpressionIR, ExpressionIR) -> ExpressionIR| {
        coverts.push(ExpressionSymbolCovert {
            symbol: tag.to_string(),
            covert: evolution,
        });
    };
    let mut register_primitives = |tag: &str, data: NativePrimitiveRender| {
        primitives.insert(tag.to_string(), Native(data))
    };
    register_symbol("?:", |e, a| {
        ItemPrimitive("get_or".to_string(), vec![e, a])
    });
    register_primitives("get_or", get_or);

    register_symbol("*", |e, a| {
        ItemPrimitive("multi".to_string(), vec![e, a])
    });
    register_primitives("multi", multi);
    register_symbol("/", |e, a| {
        ItemPrimitive("div".to_string(), vec![e, a])
    });
    register_primitives("div", div);
    register_symbol("%", |e, a| {
        ItemPrimitive("mod".to_string(), vec![e, a])
    });
    register_primitives("mod", _mod);

    register_symbol("+", |e, a| {
        ItemPrimitive("add".to_string(), vec![e, a])
    }); register_primitives("add", add);
    register_symbol("-", |e, a| {
        ItemPrimitive("sub".to_string(), vec![e, a])
    }); register_primitives("sub", sub);

    register_symbol(" is ", |e, a| {
        ItemPrimitive(
            "eq".to_string(),
            vec![ItemPrimitive("type".to_string(), vec![e]), a],
        )
    });
    register_symbol(" in ", |e, a| {
        ItemPrimitive("in".to_string(), vec![e, a])
    });
    register_symbol("==", |e, a| {
        ItemPrimitive("eq".to_string(), vec![e, a])
    });
    register_symbol("!=", |e, a| {
        ItemPrimitive(
            "not".to_string(),
            vec![ItemPrimitive("eq".to_string(), vec![e, a])],
        )
    });
    register_symbol(">=", |e, a| {
        ItemPrimitive("ge".to_string(), vec![e, a])
    });
    register_symbol("<=", |e, a| {
        ItemPrimitive("le".to_string(), vec![e, a])
    });
    register_symbol(">", |e, a| {
        ItemPrimitive("r_angle".to_string(), vec![e, a])
    });
    register_symbol("<", |e, a| {
        ItemPrimitive("l_angle".to_string(), vec![e, a])
    });
    register_symbol("&&", |e, a| {
        ItemPrimitive("and".to_string(), vec![e, a])
    });
    register_symbol("||", |e, a| {
        ItemPrimitive("or".to_string(), vec![e, a])
    });

    (coverts, primitives)
}

pub(crate) fn default_primitive_renders() -> HashMap<String, PrimitiveRenderType> {
    let mut result = HashMap::new();
    let mut register = |name: &str, func: fn(Vec<TemplateValue>) -> TmplResult<TemplateValue>| {
        result.insert(name.to_string(), Native(func))
    };
    register("get", get);
    result
}
