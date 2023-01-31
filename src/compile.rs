use std::collections::HashMap;
use std::ops::Not;
use std::rc::Rc;

use log::{debug, info};

use Expression::*;
use TemplateAst::*;

use crate::ast::{Expression, State, TemplateAst};
use crate::err::TemplateError::GenericError;
use crate::err::TmplResult;
use crate::syntax::{ChildStageType, Operator};
use crate::utils::str::{split_block, Block};
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
        let mut result = vec![];
        let block_list = split_block(
            src,
            &self.config.block_symbol.0,
            &self.config.block_symbol.1,
            &vec![("'", "'"), ("\"", "\"")],
        );
        let mut current_point = 0;
        let mut stack_list: Vec<TemplateAst> = vec![];
        // 将静态数据加入到栈中
        loop {
            if current_point >= block_list.len() {
                break;
            }
            let block = &block_list[current_point];
            debug!("block: {:?}", block);
            match *block {
                Block::Hit(hit) => {
                    let hit = hit.trim();
                    if let Some(ItemState(key, child_state, _)) = stack_list.last_mut() {
                        //存在语法块，优先寻找语法块关闭符
                        if let Operator::Branch(syntaxBlock, cjhi, _) =
                            Self::get_opera(&self.config.operator, key)?
                        {
                            let syntax_child_state = syntaxBlock
                                .child_state
                                .iter()
                                .filter(|e| hit.starts_with(e.get_tag()))
                                .map(|e| (e.get_tag(), e))
                                .collect::<Vec<(&str, &ChildStageType)>>();
                            if let Some((tag, value)) = syntax_child_state.first() {
                                if let ChildStageType::Single(opera_tag) = value {
                                    if child_state
                                        .iter()
                                        .map(|e| &e.key)
                                        .filter(|e| e == tag)
                                        .count()
                                        != 0
                                    {
                                        return Err(GenericError(format!("")));
                                    }
                                }
                                child_state.push(State::new(*tag, vec![]))
                            }
                        }
                    }
                    if let Some((key, opera)) = Self::get_match_opera(&self.config.operator, hit) {
                        let scope = match opera {
                            Operator::Branch(branch, scope, loop_state) => {
                                stack_list.push(ItemState(
                                    key.to_string(),
                                    vec![State::new(key, vec![])],
                                    *loop_state,
                                ));
                                scope
                            }
                            Operator::Command(command, scope) => {
                                Self::add_command(
                                    &mut stack_list,
                                    &mut result,
                                    ItemCommand(key.to_string(), vec![]),
                                )?;
                                scope
                            }
                        };
                        if scope.is_empty().not() {
                            //校验作用域
                            let x = stack_list.last().unwrap();
                        };
                    } else {
                        //无匹配的,识别为变量
                    }
                }
                Block::Static(value) => Self::add_static_text(&mut stack_list, &mut result, value)?,
            }
            current_point += 1;
        }
        info!("{:#?}", result);
        info!("{:#?}", stack_list);
        Ok(result)
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
                ItemState(_, stage, _) => &mut stage.last_mut().unwrap().child_stage,
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
