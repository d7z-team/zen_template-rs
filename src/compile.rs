use std::rc::Rc;

use crate::ast::TemplateAst;
use crate::compile::ast_stack::TmplAstStack;
use crate::config::TemplateConfig;
use crate::err::TmplResult;
use crate::expr::ExpressionIR;
use crate::utils::str::Block;
use crate::value::TmplValue;

pub mod ast_stack;

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
        // 将静态数据加入到栈中
        for src_block in block_arr.iter() {
            //当前块为关键字
            if let Block::Dynamic(src) = *src_block {
                //栈内存在未结束分支对象,需要考虑是否为分支内子分支或分支结束操作符
                if let Some(_branch) = stack.get_stack_top_operator(&self.config.operators)? {
                    // branch
                    // branch.syntax.child_state
                } else {
                    //栈内分支对象，直接添加根节点
                    if let Some(operator) = self.config.get_operator_by_start(src) {
                        operator.check_scope(&stack)?;
                        stack.add_node(operator.build_ast(src)?)?;
                    } else {
                        stack.add_node(TemplateAst::ItemExpr(ExpressionIR::ItemValue(
                            TmplValue::Text(src.to_string()),
                        )))?;
                    }
                }
            } else if let Block::Static(text) = *src_block {
                //当前块为静态数据
                stack.add_node(TemplateAst::new_text(text))?;
            }
        }
        println!("{:#?}", stack.root);
        println!("{:#?}", stack.child_stack);
        Ok(stack.root)
    }
}
