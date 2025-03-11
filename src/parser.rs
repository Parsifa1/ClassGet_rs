use log::info;
use serde_json::Value;

use crate::params::ValiPara;

pub async fn get_data(
    classpara: &ValiPara,
    urls: &str,
    is_tjkc: bool,
) -> anyhow::Result<serde_json::Value> {
    let url = urls.to_string() + "elective/clazz/list";
    let xg_data = r#"{"teachingClassType":"XGKC","pageNumber":1,"pageSize":1000,"orderBy":"","campus":"01","SFCT":"0"}"#;
    let tj_data = r#"{"teachingClassType":"TJKC","pageNumber":1,"pageSize":1000,"orderBy":"","campus":"01","SFCT":"0"}"#;
    let json: serde_json::Value = if is_tjkc {
        info!("正在获取专选课程列表");
        serde_json::from_str(tj_data)?
    } else {
        info!("正在获取公选课程列表");
        serde_json::from_str(xg_data)?
    };

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
            Ok(result) => {
                info!("获取课程列表成功");
                break Ok(result);
            }
            Err(e) => {
                info!("获取课程列表失败: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
        }
    }
}
