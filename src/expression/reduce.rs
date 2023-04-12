use crate::error::TmplResult;
use crate::expression::ExpressionIR;
use crate::expression::ExpressionIR::*;

impl ExpressionIR {
    ///翻译原语 (将类似 `a.to_string()` 转换为 to_string(a) )
    pub fn compile_primitives(src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        if src.len() <= 1 {
            return Ok(());
        }
        let mut result = vec![];
        while !src.is_empty() {
            let current = src.remove(0);
            let mut next_fun = || Some(src.len()).filter(|l| *l > 0).map(|_| src.remove(0));
            if let ItemSymbol(_) = current {
                result.push(current);
            } else {
                let next = next_fun();
                if let Some(ItemPrimitive(name, mut child)) = next {
                    child.insert(0, current);
                    result.push(ItemPrimitive(name.to_owned(), child));
                } else if let Some(ItemGroup(mut child)) = next {
                    child.insert(0, current);
                    result.push(ItemGroup(child));
                } else if let Some(next) = next {
                    result.push(current);
                    src.insert(0, next);
                } else {
                    result.push(current);
                }
            }
        }
        *src = result;
        for item in src {
            if let ItemPrimitive(_, child) = item {
                for item in child.iter_mut() {
                    if let ItemGroup(child) = item {
                        Self::compile_primitives(child)?;
                    }
                }
            } else if let ItemGroup(vars) = item {
                Self::compile_primitives(vars)?;
            }
        }
        Ok(())
    }
    pub fn flat_depth(src: &mut ExpressionIR) -> TmplResult<()> {
        match src {
            ItemPrimitive(_, child) => {
                Self::flat_depth_group(child)?;
            }
            ItemGroup(child) => {
                if child.len() == 1 {
                    *src = child.remove(0);
                    Self::flat_depth(src)?;
                } else {
                    Self::flat_depth_group(child)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    //深度精简
    pub fn flat_depth_group(src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        for item in src.iter_mut() {
            Self::flat_depth(item)?;
        }
        Ok(())
    }
}
