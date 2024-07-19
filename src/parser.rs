pub async fn get_data(auth: &str) -> anyhow::Result<serde_json::Value> {
    let url = "***REMOVED***elective/clazz/list";
    let data = r#"{"teachingClassType":"XGKC","pageNumber":1,"pageSize":1000,"orderBy":"","campus":"01","SFCT":"0"}"#;

    let json: serde_json::Value = serde_json::from_str(data)?;

    let json_body = loop {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("authorization", auth.parse()?);
        header.insert("batchid", "5500614d49a44ded84b68e244ae5010a".parse()?);

        let response = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(url)
            .headers(header)
            .json(&json)
            .send()
            .await?;

        let json_raw: serde_json::Value = response
            .json()
            .await
            .expect("response无法解析，请检查返回请求");

        match json_raw["code"].as_str() {
            Some("401") => continue,
            _ => break json_raw,
        }
    };

    Ok(json_body)
}
