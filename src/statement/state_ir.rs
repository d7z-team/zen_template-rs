use crate::utils::{Block, StringUtils};

pub struct DynamicBlock<'a>{
    trim_before:bool,
    trim_after:bool,
    key: &'a str,
    params: Vec<&'a str>
}
impl DynamicBlock<'static>{
    fn new<'a>(src:&'a str)->DynamicBlock<'a>{
        let mut src = src.trim();
        let mut trim = (false, false);
        // 检查 trim 选项
        if src.starts_with("- ") {
            trim.0 = true;
            src = &src[2..]
        };
        // 检查 trim 选项
        if src.ends_with(" -") {
            trim.1 = true;
            src = &src[..src.len() - 2]
        }
        src = src.trim();
        todo!()
    }
}
