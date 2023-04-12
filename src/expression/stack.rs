use crate::error::TemplateError::SyntaxError;
use crate::error::TmplResult;
use crate::expression::ExpressionIR;

pub struct ExpressionStack {
    pub(crate) data: Vec<ExpressionIR>,
}

impl Default for ExpressionStack {
    fn default() -> Self {
        ExpressionStack {
            data: vec![ExpressionIR::ItemGroup(vec![])],
        }
    }
}

impl ExpressionStack {
    pub fn push(&mut self, item: ExpressionIR)->TmplResult<()> {
        if let ExpressionIR::ItemGroup(data) = &item {
            if data.is_empty() {
                return Ok(());
            }
        }
        let result = self.data.last_mut();
        match result {
            Some(ExpressionIR::ItemGroup(value)) |
            Some(ExpressionIR::ItemPrimitive(_, value)) => {
                value.push(item)
            }
            _ => Err(SyntaxError("栈内已无等待关闭的对象，可能存在语法错误！".to_string()))?,
        }
        Ok(())
    }

    pub fn new_child(&mut self, item: ExpressionIR) {
        match &item {
            //必须是支持 child 的
            ExpressionIR::ItemGroup(_) | ExpressionIR::ItemPrimitive(_, _) => self.data.push(item),
            _ => panic!(),
        }
    }

    pub fn end_child(&mut self)->TmplResult<()> {
        let last = self.data.remove(self.data.len() - 1);
        self.push(last)
    }

    pub fn depth_len(&self) -> usize {
        self.data.len() - 1
    }
    pub fn depth(&self, index: isize) -> Option<&ExpressionIR> {
        let index = if index >= 0 {
            index as usize
        } else {
            (self.depth_len() as isize + index) as usize
        };
        self.data.get(index)
    }
    pub fn close(mut self) -> Vec<ExpressionIR> {
        if let ExpressionIR::ItemGroup(group) = self.data.remove(0) {
            group
        } else {
            panic!()
        }
    }
}
