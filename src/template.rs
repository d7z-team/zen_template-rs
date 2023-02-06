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
