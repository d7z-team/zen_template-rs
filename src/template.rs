use crate::expr::common::{ExprSymbolCovert, ExpressionIR};
use crate::syntax::*;

pub fn default_state() -> Vec<OperatorSyntax> {
    let mut result = Vec::new();
    result.push(OperatorSyntax::new_branch(
        BranchSyntax::new(
            CommandSyntax::new(
                "for",
                vec![(
                    "default",
                    vec![
                        ParamSyntax::Assignment,
                        ParamSyntax::Keywords("in".to_string()),
                        ParamSyntax::Expression,
                    ],
                )],
            ),
            vec![],
            CommandSyntax::new_empty_param("end-for"),
        ),
        vec![],
        true,
    ));
    result.push(OperatorSyntax::new_branch(
        BranchSyntax::new(
            CommandSyntax::new("switch", vec![("default", vec![ParamSyntax::Expression])]),
            vec![
                ChildStageSyntax::new_no_bind(CommandSyntax::new(
                    "case",
                    vec![("default", vec![ParamSyntax::StaticValue])],
                )),
                ChildStageSyntax::new_no_bind(CommandSyntax::new_empty_param("default")),
            ],
            CommandSyntax::new_empty_param("end-switch"),
        ),
        vec![],
        false,
    ));
    //if
    result.push(OperatorSyntax::new_branch(
        BranchSyntax::new(
            CommandSyntax::new(
                "if",
                vec![
                    ("default", vec![ParamSyntax::Expression]),
                    (
                        "let",
                        vec![ParamSyntax::Assignment, ParamSyntax::Expression],
                    ),
                ],
            ),
            vec![
                ChildStageSyntax::new_no_bind(CommandSyntax::new(
                    "else-if",
                    vec![
                        ("default", vec![ParamSyntax::Expression]),
                        (
                            "let",
                            vec![ParamSyntax::Assignment, ParamSyntax::Expression],
                        ),
                    ],
                )),
                CommandSyntax::new_empty_param("else").to_stage(vec![StageConstraint::SINGLE]),
            ],
            CommandSyntax::new_empty_param("end-if"),
        ),
        vec![],
        false,
    ));
    result.push(OperatorSyntax::new_command(
        CommandSyntax::new("include", vec![("default", vec![ParamSyntax::Expression])]),
        vec![],
    ));
    result.push(OperatorSyntax::new_command(
        CommandSyntax::new(
            "let",
            vec![(
                "default",
                vec![ParamSyntax::Assignment, ParamSyntax::Expression],
            )],
        ),
        vec![],
    ));
    result.push(OperatorSyntax::new_command(
        CommandSyntax::new("break", vec![]),
        vec!["loop", "for"],
    ));
    result.push(OperatorSyntax::new_command(
        CommandSyntax::new("continue", vec![]),
        vec!["loop", "for"],
    ));
    result
}

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
