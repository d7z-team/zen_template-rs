use std::io::ErrorKind::NotFound;
use crate::error::TemplateError::{CheckError};
use crate::error::TmplResult;
use crate::value::TemplateValue;


pub(crate) fn get_or(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    Ok(TemplateValue::Text("".to_string()))
}

pub(crate) fn get(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    check_count(&params, 2)?;
    let first = &params[0];
    let second = &params[1];
    if first == &TemplateValue::None {
        return Ok(TemplateValue::None)
    }
    if let TemplateValue::Array(arr) = first {
        //匹配 arr.usize 的情况
        if let TemplateValue::Number(index) = second {
            let i = index.clone() as usize;
            return if let Some(result) = arr.get(i) {
                Ok(result.clone())
            } else {
                Ok(TemplateValue::None)
            };
        }
    }
    todo!()
}

///检查数量
fn check_count(param: &Vec<TemplateValue>, count: usize) -> TmplResult<()> {
    if param.len() != count {
        return Err(CheckError("count".to_string(), format!("{} != {}", param.len(), count)));
    }
    Ok(())
}
