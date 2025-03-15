use std::fmt;

pub struct AsyncPara<'a> {
    pub urls: &'a str,
    pub class: &'a Vec<usize>,
    pub vali_para: &'a ValiPara,
    pub data: &'a FormatData<'a>,
}

#[derive(Clone)]
pub struct ValiPara {
    pub auth: String,
    pub batchid: String,
}

#[derive(Debug)]
pub struct ClassError {
    pub value: usize,
}

pub struct FormatData<'a> {
    pub data_json: &'a serde_json::Value,
    pub is_tjkc: bool,
}

impl fmt::Display for ClassError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an error occurred with value: {}", self.value)
    }
}
impl std::error::Error for ClassError {}
