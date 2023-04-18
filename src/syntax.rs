use std::collections::{HashMap, HashSet};
use std::ops::Not;

use log::debug;

use crate::ast::CommandParam::Keywords;
use crate::ast::TemplateAst::{ItemBranch, ItemCommand};
use crate::ast::{Branch, CommandParam, TemplateAst};
use crate::compile::ast_stack::TmplAstStack;
use crate::error::TemplateError::GenericError;
use crate::error::TmplResult;
use crate::utils::StringUtils;

/// 流程控制参数
#[derive(Debug)]
pub enum ParamSyntax {
    /// 关键字标记
    Keywords(String),
    /// 赋值标记
    Assignment,
    /// 表达式标记
    Expression,
    /// 静态数据标记，
    StaticValue,
}

/// 流程控制关键字和语法标记
#[derive(Debug)]
pub struct CommandSyntax {
    // 操作符标记
    key: String,
    // 操作符语法 (名称 / 表达式)
    syntax: HashMap<String, Vec<ParamSyntax>>,
}

impl CommandSyntax {
    ///编译表达式
    pub fn build(&self, src: &str) -> TmplResult<Branch> {
        if src.starts_with(&self.key).not() {
            return Err(GenericError(format!(
                "源码与操作符不匹配：{:?} != {:?}",
                src, self
            )));
        }
        if let Some((key, param_syntax_arr)) = self.syntax.iter().next() {
            let mut result: Vec<CommandParam> = vec![];
            let mut src = src.trim();
            for param_syntax in param_syntax_arr {
                match param_syntax {
                    ParamSyntax::Keywords(key) => {
                        if src.starts_with(&format!("{} ", key)) {
                            src = &src[key.len() + 1..];
                            result.push(Keywords)
                        } else {
                            continue;
                        }
                    }
                    ParamSyntax::Assignment => {
                        if let Some(end) = src.find('=') {
                            let param = &src[..end].trim();
                            let params = if param.starts_with('(') && param.ends_with(')') {
                                param.split(',').collect::<Vec<&str>>()
                            } else {
                                vec![*param]
                            };
                            if let Some(inv_expr) =
                                params.iter().find(|e| StringUtils::is_expr(e).not())
                            {
                                debug!("变量 {:?} 格式错误！", inv_expr);
                                continue;
                            }
                            //添加变量
                            result.push(CommandParam::Assignment(
                                params.iter().map(|e| e.to_string()).collect(),
                            ))
                        } else {
                            continue;
                        }
                    }
                    ParamSyntax::Expression => {}
                    ParamSyntax::StaticValue => {}
                }
            }
            return Ok(Branch::new(key.as_str(), result));
        }
        return Err(GenericError(format!(
            "表达式 {} 未找到匹配的解析规则！",
            src
        )));
    }
    pub fn new(tag: &str, syntax: Vec<(&str, Vec<ParamSyntax>)>) -> Self {
        CommandSyntax {
            key: tag.to_string(),
            syntax: syntax
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }
    }
    pub fn to_stage(self, bind: Vec<StageConstraint>) -> ChildStageSyntax {
        ChildStageSyntax::new(self, bind)
    }
    pub fn new_empty_param(tag: &str) -> Self {
        Self::new(tag, vec![])
    }
}

#[derive(Debug)]
pub struct ChildStageSyntax {
    tag: CommandSyntax,
    bind: HashSet<StageConstraint>,
}

impl ChildStageSyntax {
    pub fn new(tag: CommandSyntax, bind: Vec<StageConstraint>) -> Self {
        ChildStageSyntax {
            tag,
            bind: bind.into_iter().collect(),
        }
    }
    pub fn new_no_bind(tag: CommandSyntax) -> Self {
        Self::new(tag, vec![])
    }
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Hash)]
pub enum StageConstraint {
    SINGLE,
}

/// 带子流程的操作运算
#[derive(Debug)]
pub struct BranchSyntax {
    /// 开始关键字
    pub start: CommandSyntax,
    /// 子分支
    pub child_state: Vec<ChildStageSyntax>,
    /// 结束关键字
    pub end: CommandSyntax,
}

impl BranchSyntax {
    pub fn new(start: CommandSyntax, child: Vec<ChildStageSyntax>, end: CommandSyntax) -> Self {
        BranchSyntax {
            start,
            child_state: child,
            end,
        }
    }
    pub fn get_matched_type(&self, tag: &str) -> Option<&ChildStageSyntax> {
        self.child_state
            .iter()
            .find(|e| tag.starts_with(&e.tag.key))
    }
}

#[derive(Debug)]
pub struct BranchSyntaxWrapper {
    pub syntax: BranchSyntax,
    pub scopes: Vec<String>,
    pub auto_loop: bool,
}

#[derive(Debug)]
pub struct CommandSyntaxWrapper {
    pub syntax: CommandSyntax,
    pub scope: Vec<String>,
}

/// 流程操作符分类
#[derive(Debug)]
pub enum OperatorSyntax {
    /// 流程分支
    Branch(BranchSyntaxWrapper),
    /// 流程控制命令
    Command(CommandSyntaxWrapper),
}

impl OperatorSyntax {
    ///
    ///
    /// 创建新的流程
    ///
    /// # Arguments
    ///
    /// * `block`: 流程块
    /// * `scope`: 作用域
    /// * `loop_state`: 是否循环
    ///
    /// returns: Operator
    ///
    pub fn new_branch(block: BranchSyntax, scope: Vec<&str>, loop_branch: bool) -> Self {
        OperatorSyntax::Branch(BranchSyntaxWrapper {
            syntax: block,
            scopes: scope.iter().map(|e| e.to_string()).collect(),
            auto_loop: loop_branch,
        })
    }
    pub fn new_command(tag: CommandSyntax, scope: Vec<&str>) -> Self {
        OperatorSyntax::Command(CommandSyntaxWrapper {
            syntax: tag,
            scope: scope.iter().map(|e| e.to_string()).collect(),
        })
    }
    pub fn get_start_tag(&self) -> &str {
        match self {
            OperatorSyntax::Branch(item) => &item.syntax.start.key,
            OperatorSyntax::Command(item) => &item.syntax.key,
        }
    }
    pub fn get_scope(&self) -> &Vec<String> {
        match self {
            OperatorSyntax::Branch(item) => &item.scopes,
            OperatorSyntax::Command(item) => &item.scope,
        }
    }
    pub fn check_scope(&self, stack: &TmplAstStack) -> TmplResult<()> {
        stack
            .check_scope(self.get_scope())
            .map_err(|e| GenericError(format!("解析{:?} 失败，{}", self, e)))
    }
    ///将原始块
    pub fn build_ast(&self, start: &str) -> TmplResult<TemplateAst> {
        Ok(match self {
            OperatorSyntax::Branch(b) => ItemBranch(
                b.syntax.start.key.to_string(),
                vec![b.syntax.start.build(start)?],
                b.auto_loop,
            ),
            OperatorSyntax::Command(c) => {
                ItemCommand(c.syntax.key.to_string(), c.syntax.build(start)?.params)
            }
        })
    }
}
