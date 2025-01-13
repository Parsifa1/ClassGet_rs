use std::fmt;

use anyhow::anyhow;

use crate::ClassPara;

#[derive(Debug)]
pub struct ClassError {
    pub value: usize,
}

impl fmt::Display for ClassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "an error occurred with value: {}", self.value)
    }
}
impl std::error::Error for ClassError {}

pub async fn fetch_all_class(data_json: &serde_json::Value) -> anyhow::Result<Vec<String>> {
    let num = &data_json["data"]["total"].as_u64().unwrap_or(0);
    let mut formatted_strings = Vec::new();
    let num = *num as usize;
    for i in 1..num {
        let formatted_str = format!(
            "{}: {} {}",
            i, data_json["data"]["rows"][i]["KCM"], data_json["data"]["rows"][i]["XGXKLB"]
        );
        formatted_strings.push(formatted_str);
    }
    if formatted_strings.is_empty() {
        return Err(anyhow::anyhow!("未能成功获取课程列表"));
    }
    Ok(formatted_strings)
}

pub async fn get_class(
    num: usize,
    urls: String,
    classpara: ClassPara,
    data_json: serde_json::Value,
) -> anyhow::Result<()> {
    if num == 0 {
        return Ok(());
    }
    let pram = &data_json["data"]["rows"][num];

    let data = [
        ("clazzType", "XGKC"),
        ("clazzId", pram["JXBID"].as_str().unwrap_or("")),
        ("secretVal", pram["secretVal"].as_str().unwrap_or("")),
    ];

    let url = urls.to_string() + "elective/clazz/add";
    if num == 0 {
        return Ok(());
    }
    loop {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("authorization", classpara.auth.parse()?);
        header.insert("batchid", classpara.batchid.parse()?);
        let response = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(&url)
            .headers(header)
            .form(&data)
            .send()
            .await?;

        tokio::time::sleep(tokio::time::Duration::from_millis(350)).await;

        let json_body: serde_json::Value = response.json().await?;
        let kcm = &data_json["data"]["rows"][num]["KCM"]
            .as_str()
            .ok_or(anyhow!("转换失败"))?;
        let xgxklb = &data_json["data"]["rows"][num]["XGXKLB"]
            .as_str()
            .ok_or(anyhow!("转换失败"))?;
        let msg = &json_body["msg"].as_str().ok_or(anyhow!("转换失败"))?;

        match json_body["msg"].as_str() {
            Some("参数校验不通过") | Some("教学任务信息过期，请重新刷新列表") =>
            {
                let e = ClassError { value: num };
                return Err(anyhow::anyhow!(e));
            }
            Some("请求过快，请登录后再试") => {}
            Some("该课程已在选课结果中") => {
                log::info!("{} {} {} {}", "SUCSESS!", kcm, xgxklb, msg);
                break;
            }
            _ => {
                log::info!("{} {} {}", kcm, xgxklb, msg);
            }
        }
    }

    Ok(())
}
