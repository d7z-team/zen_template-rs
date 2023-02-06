use crate::expr::ExpressionIR;
use crate::expr::ExpressionIR::*;

impl ExpressionIR {
    ///翻译原语 (将类似 `a.to_string()` 转换为 to_string(a) )
    pub(crate) fn covert_primitives(src: Vec<ExpressionIR>) -> Vec<ExpressionIR> {
        let mut iter = src.into_iter();
        let mut _current = iter.next();
        let mut result = vec![];

        loop {
            let mut current = _current.unwrap();
            let mut next = iter.next();

            if let ItemPrimitive(name, child) = current {
                current = ItemPrimitive(name, Self::covert_primitives(child));
            } else if let ItemGroup(vars) = current {
                current = ItemGroup(Self::covert_primitives(vars));
            }
            if let ItemSymbol(item) = current {
                result.push(ItemSymbol(item));
            } else {
                if let Some(ItemPrimitive(name, mut child)) = next {
                    child.insert(0, current);
                    result.push(ItemPrimitive(name, child));
                    next = iter.next();
                } else {
                    result.push(current);
                }
            }
            _current = next;
            if _current.is_none() {
                break;
            }
        }
        result
    }
}
