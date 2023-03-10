use std::fmt::{Debug, Display, Formatter};

#[allow(dead_code)]
pub type TmplResult<T> = Result<T, TemplateError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TemplateError {
    StdErr(std::fmt::Error),
    GenericError(String),
    ExistsError(String),
    SyntaxError(String),
    CheckError(String,String),
}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::StdErr(err) => write!(f, "{}", err),
            TemplateError::GenericError(err) => write!(f, "{:?}", err),
            TemplateError::ExistsError(key) => write!(f, "key '{}' exists. ", key),
            TemplateError::SyntaxError(msg) => write!(f, "{}", msg),
            TemplateError::CheckError(key,msg) => write!(f, "'{}' failed: {}", key,msg),
        }
    }
}

impl From<std::fmt::Error> for TemplateError {
    fn from(value: std::fmt::Error) -> Self {
        Self::StdErr(value)
    }
}
