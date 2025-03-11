use std::fmt;
use std::sync::Arc;

pub struct AsyncPara {
    pub urls: Arc<String>,
    pub class: Arc<Vec<usize>>,
    pub classpara: Arc<ValiPara>,
    pub data: Arc<FormatData>,
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

pub struct FormatData {
    pub data_json: serde_json::Value,
    pub is_tjkc: bool,
}

impl fmt::Display for ClassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "an error occurred with value: {}", self.value)
    }
}
impl std::error::Error for ClassError {}
