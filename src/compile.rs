pub mod ast_stack;

use std::collections::HashMap;
use std::ops::Not;
use std::rc::Rc;

use log::{debug, info};

use Expression::*;
use TemplateAst::*;

use crate::ast::{Expression, Stage, TemplateAst};
use crate::compile::ast_stack::TmplAstStack;
use crate::err::TemplateError::{GenericError, SyntaxError};
use crate::err::TmplResult;
use crate::syntax::{ChildStageType, Operator};
use crate::utils::str::Block;
use crate::value::TmplValue;
use crate::TemplateConfig;

pub struct Compile {
    config: Rc<TemplateConfig>,
}

impl Compile {
    pub(crate) fn new(config: Rc<TemplateConfig>) -> Self {
        Compile { config }
    }
    pub fn build_template(&self, src: &str) -> TmplResult<Vec<TemplateAst>> {
        let mut stack = TmplAstStack::default();
        let block_arr = Block::new_group(
            src,
            &self.config.block_symbol.0,
            &self.config.block_symbol.1,
            &vec![("'", "'"), ("\"", "\"")],
        );
        let mut current_point = 0;
        // 将静态数据加入到栈中
        for (index, src_block) in block_arr.iter().enumerate() {
            if let Block::Hit(hit) = *src_block {
                //当前块为关键字
                if stack.has_stack() {}
            } else if let Block::Static(text) = *src_block {
                //当前块为静态数据
                stack.add_inline_node(TemplateAst::new_text(text))?;
            }
        }
        loop {
            if current_point >= block_arr.len() {
                break;
            }
            let block = &block_arr[current_point];
            debug!("block: {:?}", block);
            match *block {
                Block::Hit(hit) => {
                    let hit = hit.trim();
                    if let Some(ItemStage(key, child_state_arr, _)) = stack_arr.last_mut() {
                        //存在语法块，优先寻找语法块关闭符
                        if let Operator::Branch(syntaxBlock, scope, _) =
                            Self::get_opera(&self.config.operator, key)?
                        {
                            if let Some(stage_type) = syntaxBlock.get_matched_type(hit) {
                                if let ChildStageType::Single(opera_tag) = stage_type {
                                    if let Some(_) = child_state_arr
                                        .iter()
                                        .find(|e| e.key == stage_type.get_tag())
                                    {
                                        return Err(SyntaxError(format!(
                                            "语法错误，此分支已出现相同路径 {:?}.",
                                            child_state_arr
                                        )));
                                    }
                                }
                                child_state_arr.push(Stage::new(stage_type.get_tag(), vec![]))
                            }
                        }
                    }
                    if let Some((key, opera)) = Self::get_match_opera(&self.config.operator, hit) {
                        let scope = match opera {
                            Operator::Branch(branch, scope, loop_state) => {
                                stack_arr.push(ItemStage(
                                    key.to_string(),
                                    vec![Stage::new(key, vec![])],
                                    *loop_state,
                                ));
                                scope
                            }
                            Operator::Command(command, scope) => {
                                Self::add_command(
                                    &mut stack_arr,
                                    &mut result,
                                    ItemCommand(key.to_string(), vec![]),
                                )?;
                                scope
                            }
                        };
                        if scope.is_empty().not() {
                            //校验作用域
                            let x = stack_arr.last().unwrap();
                        };
                    } else {
                        //无匹配的,识别为变量
                    }
                }
                Block::Static(value) => Self::add_static_text(&mut stack_arr, &mut result, value)?,
            }
            current_point += 1;
        }

        Ok(stack.root)
    }
    fn get_opera<'a>(operators: &'a Vec<Operator>, hit_str: &str) -> TmplResult<&'a Operator> {
        let map = operators
            .iter()
            .filter(|e| e.get_start_tag() == hit_str)
            .map(|e| (e.get_start_tag(), e))
            .collect::<Vec<(&str, &Operator)>>();
        map.first()
            .map(|e| e.1)
            .ok_or(GenericError(format!("未发现名为 {} 的匹配", hit_str)))
    }
    ///查询可用的控制器
    fn get_match_opera<'a>(
        operators: &'a Vec<Operator>,
        hit_str: &str,
    ) -> Option<(&'a str, &'a Operator)> {
        let map = operators
            .iter()
            .filter(|current| hit_str.starts_with(&format!("{} ", current.get_start_tag())))
            .map(|e| (e.get_start_tag(), e))
            .collect::<Vec<(&str, &Operator)>>();
        map.first().map(|e| (e.0, e.1))
    }

    fn get_current_stack<'a>(
        stack: &'a mut Vec<TemplateAst>,
        main: &'a mut Vec<TemplateAst>,
    ) -> TmplResult<&'a mut Vec<TemplateAst>> {
        Ok(if let Some(last_ast) = stack.last_mut() {
            match last_ast {
                ItemStage(_, stage, _) => &mut stage.last_mut().unwrap().child_stage,
                e => Err(GenericError(format!(
                    "BUG：栈中永远不会包含不支持嵌套的对象({:#?})！",
                    e
                )))?,
            }
        } else {
            main
        })
    }
    fn add_command(
        stack: &mut Vec<TemplateAst>,
        main: &mut Vec<TemplateAst>,
        node: TemplateAst,
    ) -> TmplResult<()> {
        Self::get_current_stack(stack, main)?.push(node);
        return Ok(());
    }
    fn add_static_text(
        stack: &mut Vec<TemplateAst>,
        main: &mut Vec<TemplateAst>,
        src: &str,
    ) -> TmplResult<()> {
        let content = Self::get_current_stack(stack, main)?;
        if let Some(ItemExpr(ItemStatic(TmplValue::Text(text)))) = content.last_mut() {
            text.push_str(src)
        } else {
            content.push(ItemExpr(ItemStatic(TmplValue::Text(src.to_string()))))
        }
        Ok(())
    }
}
