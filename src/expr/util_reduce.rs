use crate::err::TmplResult;
use crate::expr::ExpressionIR;
use crate::expr::ExpressionIR::*;

impl ExpressionIR {
    ///翻译原语 (将类似 `a.to_string()` 转换为 to_string(a) )
    pub fn compile_primitives(src: &mut Vec<ExpressionIR>) -> TmplResult<()> {
        if src.len() <= 1 {
            return Ok(());
        }
        let mut result = vec![];
        while src.len() >= 1 {
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
                Self::compile_primitives(child)?;
            } else if let ItemGroup(vars) = item {
                Self::compile_primitives(vars)?;
            }
        }
        Ok(())
    }
}
