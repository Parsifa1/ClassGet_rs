use serde_json::Value;

use crate::ClassPara;

pub async fn get_data(classpara: &ClassPara) -> anyhow::Result<serde_json::Value> {
    let url = "***REMOVED***elective/clazz/list";
    let data = r#"{"teachingClassType":"XGKC","pageNumber":1,"pageSize":1000,"orderBy":"","campus":"01","SFCT":"0"}"#;

    let json: serde_json::Value = serde_json::from_str(data)?;

    let json_body = loop {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("authorization", classpara.auth.parse()?);
        header.insert("batchid", classpara.batchid.parse()?);

        let response = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(url)
            .headers(header)
            .json(&json)
            .send()
            .await?;

        match response.json::<Value>().await {
            Ok(json) => {
                if let Some(code) = json["code"].as_str() {
                    if code == "401" {
                        continue; // 如果 code 是 401，继续循环
                    }
                }
                break json; // 如果 code 不是 401，退出循环并返回 json
            }
            Err(_) => continue, // 失败时继续循环
        }
    };

    Ok(json_body)
}
