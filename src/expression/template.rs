mod primitive;

use primitive::*;
use std::collections::HashMap;
use ExpressionIR::ItemPrimitive;
use crate::error::TmplResult;
use crate::expression::{ExpressionIR, ExpressionSymbolCovert, PrimitiveRenderType};
use crate::expression::PrimitiveRenderType::Native;
use crate::value::TemplateValue;

/// 表达式符号映射表，将相关的表达式转换为原语
pub fn default_expressions_symbol() -> (Vec<ExpressionSymbolCovert>, HashMap<String, PrimitiveRenderType>) {
    let mut coverts = Vec::new();
    let mut primitives = HashMap::new();

    let mut register = |tag: &str, evolution: fn(ExpressionIR, ExpressionIR) -> ExpressionIR, types: PrimitiveRenderType| {
        coverts.push(ExpressionSymbolCovert {
            symbol: tag.to_string(),
            covert: evolution,
        });
        primitives.insert(tag.to_string(), types);
    };

    register("?:", |e, a| {
        ItemPrimitive("get_or".to_string(), vec![e, a])
    }, Native(get_or));

    register("*", |e, a| {
        ItemPrimitive("multi".to_string(), vec![e, a])
    }, Native(get_or));
    register("/", |e, a| {
        ItemPrimitive("div".to_string(), vec![e, a])
    }, Native(get_or));
    register("%", |e, a| {
        ItemPrimitive("mod".to_string(), vec![e, a])
    }, Native(get_or));

    register("+", |e, a| {
        ItemPrimitive("add".to_string(), vec![e, a])
    }, Native(get_or));
    register("-", |e, a| {
        ItemPrimitive("sub".to_string(), vec![e, a])
    }, Native(get_or));

    register(" is ", |e, a| {
        ItemPrimitive(
            "eq".to_string(),
            vec![ItemPrimitive("type".to_string(), vec![e]), a],
        )
    }, Native(get_or));
    register(" in ", |e, a| {
        ItemPrimitive("in".to_string(), vec![e, a])
    }, Native(get_or));
    register("==", |e, a| {
        ItemPrimitive("eq".to_string(), vec![e, a])
    }, Native(get_or));
    register("!=", |e, a| {
        ItemPrimitive(
            "not".to_string(),
            vec![ItemPrimitive("eq".to_string(), vec![e, a])],
        )
    }, Native(get_or));
    register(">=", |e, a| {
        ItemPrimitive("ge".to_string(), vec![e, a])
    }, Native(get_or));
    register("<=", |e, a| {
        ItemPrimitive("le".to_string(), vec![e, a])
    }, Native(get_or));
    register(">", |e, a| {
        ItemPrimitive("r_angle".to_string(), vec![e, a])
    }, Native(get_or));
    register("<", |e, a| {
        ItemPrimitive("l_angle".to_string(), vec![e, a])
    }, Native(get_or));
    register("&&", |e, a| {
        ItemPrimitive("and".to_string(), vec![e, a])
    }, Native(get_or));
    register("||", |e, a| {
        ItemPrimitive("or".to_string(), vec![e, a])
    }, Native(get_or));

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
