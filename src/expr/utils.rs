use crate::expr::ExpressCompileIR::{Original, Tag};
use crate::expr::ExpressionIR;
use crate::expr::ExpressionIR::ItemValue;
use crate::utils::str::Block;
use crate::value::TmplValue;

///表达式编译中间产物
#[derive(Debug, PartialEq)]
pub enum ExpressCompileIR<'a> {
    //带标记的处理数据
    Tag(ExpressionIR),
    //不带标记的原始数据
    Original(&'a str),
}

impl ExpressCompileIR<'static> {
    //源码下的原始字符串提取出来
    pub fn parse_static_str(src: &str) -> Vec<ExpressCompileIR> {
        Block::new_group(src.trim(), "\"", "\"", &vec![("'", "'")])
            .into_iter()
            .flat_map(|e| match e {
                Block::Static(d) => Block::new_group(d, "'", "'", &vec![("\"", "\"")])
                    .into_iter()
                    .map(|e| match e {
                        Block::Dynamic(dy) => Tag(ItemValue(TmplValue::Text(dy.to_string()))),
                        Block::Static(st) => Original(st),
                    })
                    .collect(),
                Block::Dynamic(s) => vec![Tag(ItemValue(TmplValue::Text(s.to_string())))],
            })
            .collect::<Vec<ExpressCompileIR>>()
    }
}

#[cfg(test)]
mod test {
    use crate::expr::ExpressCompileIR;
    use crate::expr::ExpressCompileIR::{Original, Tag};
    use crate::expr::ExpressionIR::ItemValue;
    use crate::value::TmplValue;

    #[test]
    fn test_parse_str() {
        assert_eq!(
            ExpressCompileIR::parse_static_str(r#"hello world 'dragon' "dragon""#),
            vec![
                Original("hello world "),
                Tag(ItemValue(TmplValue::Text("dragon".to_string()))),
                Original(" "),
                Tag(ItemValue(TmplValue::Text("dragon".to_string()))),
            ]
        )
    }
}
