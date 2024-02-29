use std::fmt;
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

pub async fn print_all_class(data_json: &serde_json::Value) -> anyhow::Result<Vec<String>> {
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
    auth: String,
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

    let url = "***REMOVED***elective/clazz/add";
    if num == 0 {
        return Ok(());
    }
    loop {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("authorization", auth.parse()?);
        header.insert("batchid", "5500614d49a44ded84b68e244ae5010a".parse()?);
        let response = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(url)
            .headers(header)
            .form(&data)
            .send()
            .await?;

        let json_body: serde_json::Value = response.json().await?;
        let kcm = &data_json["data"]["rows"][num]["KCM"];
        let xgxklb = &data_json["data"]["rows"][num]["XGXKLB"];
        let msg = &json_body["msg"];

        if json_body["msg"] == "参数校验不通过" {
            let e = ClassError { value: num };
            return Err(anyhow::anyhow!(e));
        }
        if json_body["msg"] == "教学任务信息过期，请重新刷新列表" {
            let e = ClassError { value: num };
            return Err(anyhow::anyhow!(e));
        }
        if json_body["msg"] != "请求过快，请登录后再试" {
            println!("{} {}", kcm, xgxklb);
            println!("{}", msg);
        }
        if json_body["msg"] == "该课程已在选课结果中" {
            log::info!("{}", msg);
            log::info!("{} {}", kcm, xgxklb);
            break;
        }
    }

    Ok(())
}
