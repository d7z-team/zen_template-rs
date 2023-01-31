use crate::ast::Expression::ItemStatic;
use crate::ast::TemplateAst;
use crate::ast::TemplateAst::ItemStage;
use crate::err::TemplateError::{ExistsError, GenericError};
use crate::err::TmplResult;
use crate::value::TmplValue;
use std::ops::Not;
use TemplateAst::ItemExpr;

pub struct TmplAstStack {
    pub(crate) root: Vec<TemplateAst>,
    child_stack: Vec<TemplateAst>,
}

impl TmplAstStack {
    ///
    /// 将控制对象添加至根节点或栈内
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
    pub fn add_inline_node(&mut self, node: TemplateAst) -> TmplResult<()> {
        if let ItemStage(..) = node {
            return Err(GenericError(format!("只允许添加控制对象！")));
        }
        let add = if self.child_stack.is_empty() {
            &mut self.root
        } else if let Some(ItemStage(_, stage, _)) = self.child_stack.last_mut() {
            &mut stage
                .last_mut()
                .ok_or(ExistsError(
                    "BUG：分支对象不存在默认的子分支，请排查入栈相关的代码".to_string(),
                ))?
                .child_stage
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
        Ok(())
    }
    ///栈内存在未关闭的分支对象
    pub fn has_stack(&self) -> bool {
        self.child_stack.is_empty().not()
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
