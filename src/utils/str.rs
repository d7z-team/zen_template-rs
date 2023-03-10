use std::ops::Not;

pub struct StringUtils {}

impl StringUtils {
    /// 查询字符串
    pub fn find(src: &str, begin: usize, tag: &str) -> Option<usize> {
        let start = begin;
        if let Some(block_start) = src[start..].find(tag).map(|e| e + start) {
            Some(block_start)
        } else {
            None
        }
    }

    pub fn find_block(
        src: &str,
        index: usize,
        start_tag: &str,
        end_tag: &str,
    ) -> Option<(usize, Option<usize>)> {
        let start = Self::find(src, index, start_tag)?;
        Some((
            start,
            Self::find(src, start + start_tag.len(), end_tag).map(|e| e + end_tag.len()),
        ))
    }

    /// 查询文本块，并且排除忽略的文本块
    pub fn find_block_skip_ignore(
        src: &str,
        index: usize,
        start_tag: &str,
        end_tag: &str,
        ignore_block: &Vec<(&str, &str)>,
    ) -> Option<(usize, usize)> {
        // 如果找不到文本块或者找到的文本块只有开始，没有结束则返回 None
        let (start, mut end) = Self::find_block(src, index, start_tag, end_tag)
            .filter(|e| e.1.is_some())
            .map(|e| (e.0, e.1.unwrap()))?;
        loop {
            let text_block = &src[start + start_tag.len()..end - end_tag.len()];
            let mut ignore_range = ignore_block
                .iter()
                .map(|e| Self::find_block(text_block, 0, e.0, e.1))
                .filter(|e| e.is_some())
                .map(|e| e.unwrap())
                .collect::<Vec<(usize, Option<usize>)>>();
            ignore_range.sort_by_key(|e| e.0);
            let mut current_range_index = 0;
            loop {
                if let Some((current_start, current_end)) = ignore_range
                    .get(current_range_index)
                    .map(|e| (e.0, e.1.unwrap_or(text_block.len())))
                {
                    // 移除重叠的区块
                    let mut remove = ignore_range
                        .iter()
                        .enumerate()
                        .filter(|(_, (start, _))| current_start < *start && current_end > *start)
                        .map(|e| e.0)
                        .collect::<Vec<usize>>();
                    remove.sort();
                    remove.reverse();
                    for id in remove {
                        ignore_range.remove(id);
                    }
                } else {
                    break;
                }
                current_range_index += 1;
            }
            if ignore_range.iter().any(|e| e.1.is_none()) {
                // 子文本块未结束，继续查询下一个
                if end + 1 >= src.len() {
                    return None;
                }
                end = Self::find(src, end + 1, end_tag).map(|e| e + end_tag.len())?;
            } else {
                break;
            }
        }
        Some((start, end))
    }

    pub fn is_expr(src: &str) -> bool {
        let chars = src.chars().collect::<Vec<char>>();
        let first = chars[0];
        if (('A'..='Z').contains(&first) || ('a'..='z').contains(&first) || first == '_').not() {
            return false;
        }
        for item in chars.iter() {
            if (('A'..='Z').contains(&item)
                || ('a'..='z').contains(&item)
                || ('0'..='9').contains(&item)
                || *item == '_')
                .not()
            {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Block<'a> {
    Dynamic(&'a str),
    Static(&'a str),
}

impl Block<'static> {
    pub fn new_group<'a>(
        src: &'a str,
        tag: (&str, &str),
        ignore_blocks: &Vec<(&str, &str)>,
    ) -> Vec<Block<'a>> {
        let mut result = vec![];
        let mut index = 0;
        loop {
            if let Some((current_start, current_end)) =
                StringUtils::find_block_skip_ignore(src, index, tag.0, tag.1, ignore_blocks)
            {
                if current_start > index {
                    result.push(Block::Static(&src[index..current_start]));
                }
                let dyn_src = &src[current_start + tag.0.len()..current_end - tag.1.len()];
                result.push(Block::Dynamic(dyn_src));
                index = current_end
            } else {
                if index < src.len() {
                    result.push(Block::Static(&src[index..]));
                }
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::utils::StringUtils;

    #[test]
    fn test_find() {
        assert_eq!(StringUtils::find("hello world", 1, "world"), Some(6));
        assert_eq!(StringUtils::find("hello \\world", 1, "world"), Some(7));
        assert_eq!(
            StringUtils::find("hello \\world world", 1, "world"),
            Some(7)
        );
    }

    #[test]
    fn test_find_block() {
        assert_eq!(
            StringUtils::find_block("Hello {{world}}", 0, "{{", "}}"),
            Some((6, Some(15)))
        );
        assert_eq!(
            StringUtils::find_block("Hello {{world", 0, "{{", "}}"),
            Some((6, None))
        );
        assert_eq!(
            StringUtils::find_block("Hello world}}", 0, "{{", "}}"),
            None
        );
    }

    #[test]
    fn test_find_nesting_block() {
        fn get_expr(src: &str) -> Option<&str> {
            StringUtils::find_block_skip_ignore(src, 0, "{{", "}}", &vec![("\"", "\""), ("'", "'")])
                .map(|e| &src[e.0..e.1])
        }
        assert_eq!(
            get_expr(r#"hello {{ self.data + current.context + 'item}}' + "" }}"#),
            Some(r#"{{ self.data + current.context + 'item}}' + "" }}"#)
        );
        assert_eq!(
            get_expr(r#"hello {{ self.data + current.context + 'item}}' + " }}"#),
            None
        );
        assert_eq!(get_expr(r#"hello world"#), None);
        assert_eq!(get_expr(r#"hello {{world}}"#), Some("{{world}}"));
        assert_eq!(get_expr(r#"hello {{wo}}rld}}"#), Some("{{wo}}"));
        assert_eq!(get_expr("hello {{wo\nrld}}"), Some("{{wo\nrld}}"));
        assert_eq!(get_expr("hello好{{wo\nrld}}"), Some("{{wo\nrld}}"));
    }
}
