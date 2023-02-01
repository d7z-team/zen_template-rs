use std::collections::HashSet;
use std::ops::Not;

use TemplateAst::ItemExpr;

use crate::ast::TemplateAst;
use crate::ast::TemplateAst::ItemBranch;
use crate::err::TemplateError::{ExistsError, GenericError};
use crate::err::TmplResult;
use crate::expr::Expression::ItemStatic;
use crate::syntax::{BranchSyntaxWrapper, OperatorSyntax};
use crate::value::TmplValue;

pub struct TmplAstStack {
    pub root: Vec<TemplateAst>,
    pub child_stack: Vec<TemplateAst>,
}

impl TmplAstStack {
    ///
    /// 将对象添加至根节点或栈内
    ///
    /// # Arguments
    ///
    /// * `node`:
    ///
    /// returns: Result<(), TemplateError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add_node(&mut self, node: TemplateAst) -> TmplResult<()> {
        if let ItemBranch(..) = node {
            self.child_stack.push(node)
        } else {
            let add = if self.child_stack.is_empty() {
                &mut self.root
            } else if let Some(ItemBranch(_, stage, _)) = self.child_stack.last_mut() {
                &mut stage
                    .last_mut()
                    .ok_or(ExistsError(
                        "BUG：分支对象不存在默认的子分支，请排查入栈相关的代码".to_string(),
                    ))?
                    .child_stages
            } else {
                Err(GenericError(format!(
                    "BUG：栈中永远不会包含控制对象,当前栈信息如下：({:#?})！",
                    self.child_stack
                )))?
            };
            if let ItemExpr(ItemStatic(TmplValue::Text(new))) = node {
                if let Some(ItemExpr(ItemStatic(TmplValue::Text(old)))) = &mut add.last_mut() {
                    old.push_str(&new)
                } else {
                    add.push(ItemExpr(ItemStatic(TmplValue::Text(new))))
                }
            } else {
                add.push(node)
            }
        }

        Ok(())
    }
    ///栈内存在未关闭的分支对象
    pub fn has_stack(&self) -> bool {
        self.child_stack.is_empty().not()
    }

    pub fn last_mut_stack(&mut self) -> Option<&mut TemplateAst> {
        self.child_stack.last_mut()
    }
    ///获取栈顶操作符类型
    pub fn get_stack_top_operator<'a>(
        &self,
        operator_arr: &'a Vec<OperatorSyntax>,
    ) -> TmplResult<Option<&'a BranchSyntaxWrapper>> {
        if let Some(ItemBranch(key, _, _)) = self.child_stack.last() {
            operator_arr
                .iter()
                .find(|e| e.get_start_tag() == key)
                .ok_or(GenericError(format!("未发现名为 {} 的匹配", key)))
                .map(|e| {
                    if let OperatorSyntax::Branch(b) = e {
                        Some(b)
                    } else {
                        panic!("栈内对象永远不可能为控制对象！")
                    }
                })
        } else {
            Ok(None)
        }
    }
    pub fn check_scope(&self, scope: &Vec<String>) -> TmplResult<()> {
        let context = self
            .child_stack
            .iter()
            .map(|e| e.get_tag().unwrap())
            .collect::<HashSet<&str>>();
        let not_matched = scope
            .iter()
            .filter(|e| context.contains(e.as_str()).not())
            .map(|e| e)
            .collect::<Vec<&String>>();
        if not_matched.is_empty().not() {
            Err(GenericError(format!("{:?} 不在作用域内", not_matched)))
        } else {
            Ok(())
        }
    }
}

impl Default for TmplAstStack {
    fn default() -> Self {
        TmplAstStack {
            root: vec![],
            child_stack: vec![],
        }
    }
}
