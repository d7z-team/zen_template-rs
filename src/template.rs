use crate::expr::*;
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
    let mut register = |tag: &str, evolution: fn(Expression, Expression) -> Primitive| {
        result.push(ExprSymbolCovert {
            symbol: tag.to_string(),
            covert: evolution,
        })
    };
    register("?:", |e, a| Primitive::new("get_or", vec![e, a]));

    register("*", |e, a| Primitive::new("multi", vec![e, a]));
    register("/", |e, a| Primitive::new("div", vec![e, a]));
    register("%", |e, a| Primitive::new("mod", vec![e, a]));

    register("+", |e, a| Primitive::new("add", vec![e, a]));
    register("-", |e, a| Primitive::new("sub", vec![e, a]));

    register(" is ", |e, a| {
        Primitive::new(
            "eq",
            vec![Expression::ItemDynamic(Primitive::new("type", vec![e])), a],
        )
    });
    register(" in ", |e, a| Primitive::new("in", vec![e, a]));
    register("==", |e, a| Primitive::new("eq", vec![e, a]));
    register("!=", |e, a| {
        Primitive::new(
            "not",
            vec![Expression::ItemDynamic(Primitive::new("eq", vec![e, a]))],
        )
    });
    register(">=", |e, a| Primitive::new("ge", vec![e, a]));
    register("<=", |e, a| Primitive::new("le", vec![e, a]));
    register(">", |e, a| Primitive::new("r_angle", vec![e, a]));
    register("<", |e, a| Primitive::new("l_angle", vec![e, a]));
    register("&&", |e, a| Primitive::new("and", vec![e, a]));
    register("||", |e, a| Primitive::new("or", vec![e, a]));

    result
}
