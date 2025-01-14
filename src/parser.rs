use log::info;
use serde_json::Value;

use crate::ClassPara;

pub async fn get_data(classpara: &ClassPara, urls: &str) -> anyhow::Result<serde_json::Value> {
    let url = urls.to_string() + "elective/clazz/list";
    let data = r#"{"teachingClassType":"XGKC","pageNumber":1,"pageSize":1000,"orderBy":"","campus":"01","SFCT":"0"}"#;

    let json: serde_json::Value = serde_json::from_str(data)?;

    loop {
        match async {
            let mut header = reqwest::header::HeaderMap::new();
            header.insert("authorization", classpara.auth.parse()?);
            header.insert("batchid", classpara.batchid.parse()?);

            let response = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap()
                .post(&url)
                .headers(header)
                .json(&json)
                .send()
                .await?;

            let json_response = response.json::<Value>().await?;

            if let Some(code) = json_response["code"].as_str() {
                if code == "401" {
                    anyhow::bail!("认证失败(401)");
                }
            }
            Ok(json_response)
        }
        .await
        {
            Ok(result) => break Ok(result),
            Err(e) => {
                info!("获取课程列表失败: {}，正在重试...", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
        }
    }
}
