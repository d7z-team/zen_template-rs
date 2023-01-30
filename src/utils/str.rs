use std::slice::from_raw_parts;
use std::str::from_utf8;

/// 查询字符串，并跳过 ` \ ` 标记
pub fn find(src: &str, begin: usize, tag: &str) -> Option<usize> {
    let mut start = begin;
    loop {
        if let Some(block_start) = src[start..].find(tag).map(|e| e + start) {
            if block_start > 0 && &src.as_bytes()[(block_start - 1)..block_start] == "\\".as_bytes()
            {
                //跳过无关匹配
                start = block_start + tag.len();
            } else {
                //匹配成功，
                start = block_start;
                break;
            }
        } else {
            return None;
        }
    }
    Some(start)
}

pub fn find_block(
    src: &str,
    index: usize,
    start_tag: &str,
    end_tag: &str,
) -> Option<(usize, Option<usize>)> {
    let start = find(src, index, start_tag)?;
    Some((
        start,
        find(src, start + start_tag.len(), end_tag).map(|e| e + end_tag.len()),
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
    let (start, mut end) = find_block(src, index, start_tag, end_tag)
        .filter(|e| e.1.is_some())
        .map(|e| (e.0, e.1.unwrap()))?;
    loop {
        let text_block = &src[start + start_tag.len()..end - end_tag.len()];
        let mut ignore_range = ignore_block
            .iter()
            .map(|e| find_block(text_block, 0, e.0, e.1))
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
            end = find(src, end + 1, end_tag).map(|e| e + end_tag.len())?;
        } else {
            break;
        }
    }
    Some((start, end))
}

#[derive(Debug, Eq, PartialEq)]
pub enum Block<'a> {
    Hit(&'a str),
    Static(&'a str),
}

pub fn split_block<'a>(
    src: &'a str,
    start_tag: &str,
    end_tag: &str,
    ignore_block: &Vec<(&str, &str)>,
) -> Vec<Block<'a>> {
    let mut result = vec![];
    let mut index = 0;
    loop {
        if let Some((current_start, current_end)) =
            find_block_skip_ignore(src, index, start_tag, end_tag, ignore_block)
        {
            if current_start > index {
                remove_skip_block(&src[index..current_start], start_tag)
                    .iter()
                    .flat_map(|e| remove_skip_block(*e, end_tag))
                    .map(|e| Block::Static(e))
                    .for_each(|e| result.push(e))
            }
            result.push(Block::Hit(
                &src[current_start + start_tag.len()..current_end - end_tag.len()],
            ));
            index = current_end
        } else {
            if index < src.len() {
                remove_skip_block(&src[index..], start_tag)
                    .iter()
                    .flat_map(|e| remove_skip_block(*e, end_tag))
                    .map(|e| Block::Static(e))
                    .for_each(|e| result.push(e))
            }
            break;
        }
    }
    result
}

//TODO: 覆盖
pub fn remove_skip_block<'a>(src: &'a str, tag: &str) -> Vec<&'a str> {
    let skip_tag = format!("\\{}", tag);
    let mut result: Vec<&str> = src.split(&skip_tag).collect();
    if result.len() > 1 {
        for index in 0..result.len() - 1 {
            let current: &str = result.get(index + 1).unwrap();
            let current_ptr = current.as_ptr();
            result[index + 1] = unsafe {
                from_utf8(from_raw_parts(
                    src.as_ptr().offset(
                        (current_ptr.offset_from(src.as_ptr()) - (tag.len() as isize)) as isize,
                    ),
                    tag.len() + current.len(),
                ))
                .unwrap()
            };
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::utils::str::Block::{Hit, Static};
    use crate::utils::str::{find, find_block, find_block_skip_ignore, split_block, Block};

    #[test]
    fn test_find() {
        assert_eq!(find("hello world", 1, "world"), Some(6));
        assert_eq!(find("hello \\world", 1, "world"), None);
        assert_eq!(find("hello \\world world", 1, "world"), Some(13));
    }

    #[test]
    fn test_find_block() {
        assert_eq!(
            find_block("Hello {{world}}", 0, "{{", "}}"),
            Some((6, Some(15)))
        );
        assert_eq!(
            find_block("Hello {{world\\}}", 0, "{{", "}}"),
            Some((6, None))
        );
        assert_eq!(find_block("Hello \\{{world}}", 0, "{{", "}}"), None);
    }

    #[test]
    fn test_find_nesting_block() {
        fn get_expr(src: &str) -> Option<&str> {
            find_block_skip_ignore(src, 0, "{{", "}}", &vec![("\"", "\""), ("'", "'")])
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
        assert_eq!(get_expr(r#"hello {{wo\}}rld}}"#), Some("{{wo\\}}rld}}"));
        assert_eq!(get_expr("hello {{wo\nrld}}"), Some("{{wo\nrld}}"));
        assert_eq!(get_expr("hello好{{wo\nrld}}"), Some("{{wo\nrld}}"));
    }

    #[test]
    fn test_split_block() {
        fn split_block_test(src: &str) -> Vec<Block> {
            split_block(src, "{{", "}}", &vec![("\"", "\""), ("'", "'")])
        }
        assert_eq!(
            split_block_test("hello {{world}}"),
            vec![Static("hello "), Hit("world")]
        );
        assert_eq!(
            split_block_test("hello \\{{ {{world}}"),
            vec![Static("hello "), Static("{{ "), Hit("world")]
        );

        assert_eq!(
            split_block_test("hello \\}} {{world}}"),
            vec![Static("hello "), Static("}} "), Hit("world")]
        );
        assert_eq!(
            split_block_test("hello \\}} {{world\\}}}}"),
            vec![Static("hello "), Static("}} "), Hit("world\\}}")]
        );
    }
}
