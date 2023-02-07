use crate::error::TemplateError::{CheckError};
use crate::error::TmplResult;
use crate::value::TemplateValue;


pub(crate) fn multi(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    let (first, second) = two_params(&params)?;
    if let TemplateValue::Number(num1) = first {
        if let TemplateValue::Number(num2) = second {
            return Ok(TemplateValue::Number(num1 * num2));
        }
    }
    if let TemplateValue::Float(num1) = first {
        if let TemplateValue::Float(num2) = second {
            return Ok(TemplateValue::Float(num1 * num2));
        }
    }
    Err(CheckError(format!("双方类型不一致"), format!("{:?} != {:?}", first, second)))
}

pub(crate) fn add(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    let (first, second) = two_params(&params)?;
    if *first == TemplateValue::None || *second == TemplateValue::None {
        return Ok(TemplateValue::None);
    }
    if let TemplateValue::Number(num1) = first {
        if let TemplateValue::Number(num2) = second {
            return Ok(TemplateValue::Number(num1 + num2));
        }
    }
    if let TemplateValue::Float(num1) = first {
        if let TemplateValue::Float(num2) = second {
            return Ok(TemplateValue::Float(num1 + num2));
        }
    }
    if let TemplateValue::Text(text) = first {
        let mut news = text.clone();
        news.push_str(&second.to_string());
        return Ok(TemplateValue::Text(news));
    }
    Err(CheckError(format!("双方类型不一致"), format!("{:?} != {:?}", first, second)))
}

pub(crate) fn sub(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    let (first, second) = two_params(&params)?;
    if *first == TemplateValue::None || *second == TemplateValue::None {
        return Ok(TemplateValue::None);
    }
    if let TemplateValue::Number(num1) = first {
        if let TemplateValue::Number(num2) = second {
            return Ok(TemplateValue::Number(num1 - num2));
        }
    }
    if let TemplateValue::Float(num1) = first {
        if let TemplateValue::Float(num2) = second {
            return Ok(TemplateValue::Float(num1 - num2));
        }
    }
    Err(CheckError(format!("双方类型不一致"), format!("{:?} != {:?}", first, second)))
}

pub(crate) fn _mod(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    let (first, second) = two_params(&params)?;
    if *first == TemplateValue::None || *second == TemplateValue::None {
        return Ok(TemplateValue::None);
    }
    if let TemplateValue::Number(num1) = first {
        if let TemplateValue::Number(num2) = second {
            return Ok(TemplateValue::Number(num1 % num2));
        }
    }
    if let TemplateValue::Float(num1) = first {
        if let TemplateValue::Float(num2) = second {
            return Ok(TemplateValue::Float(num1 % num2));
        }
    }
    Err(CheckError(format!("双方类型不一致"), format!("{:?} != {:?}", first, second)))
}

pub(crate) fn div(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    let (first, second) = two_params(&params)?;
    if *first == TemplateValue::None || *second == TemplateValue::None {
        return Ok(TemplateValue::None);
    }
    if let TemplateValue::Number(num1) = first {
        if let TemplateValue::Number(num2) = second {
            return Ok(TemplateValue::Number(num1 / num2));
        }
    }
    if let TemplateValue::Float(num1) = first {
        if let TemplateValue::Float(num2) = second {
            return Ok(TemplateValue::Float(num1 / num2));
        }
    }
    Err(CheckError(format!("双方类型不一致"), format!("{:?} != {:?}", first, second)))
}

pub(crate) fn get_or(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    let (first, second) = two_params(&params)?;
    if *first == TemplateValue::None || *second == TemplateValue::None {
        return Ok(TemplateValue::None);
    }
    return if *first == TemplateValue::None {
        Ok(second.to_owned())
    } else {
        Ok(first.to_owned())
    };
}

pub(crate) fn get(params: Vec<TemplateValue>) -> TmplResult<TemplateValue> {
    let (first, second) = two_params(&params)?;
    if *first == TemplateValue::None || *second == TemplateValue::None {
        return Ok(TemplateValue::None);
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
    if let TemplateValue::Table(table) = first {
        if let TemplateValue::Text(key) = second {
            if let Some(data) = table.get(key.as_str()) {
                return Ok(data.clone());
            }
        }
    }
    Ok(TemplateValue::None)
}


fn two_params(params: &Vec<TemplateValue>) -> TmplResult<(&TemplateValue, &TemplateValue)> {
    check_count(params, 2)?;
    Ok((&params[0], &params[1]))
}

///检查数量
fn check_count(param: &Vec<TemplateValue>, count: usize) -> TmplResult<()> {
    if param.len() != count {
        return Err(CheckError("count".to_string(), format!("{} != {}", param.len(), count)));
    }
    Ok(())
}
