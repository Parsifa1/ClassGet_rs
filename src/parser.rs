pub async fn get_data(auth: &str) -> anyhow::Result<serde_json::Value> {
    let url = "http://jwxk.hrbeu.edu.cn/xsxk/elective/clazz/list";
    let data = "{\"teachingClassType\":\"XGKC\",\"pageNumber\":1,\"pageSize\":1000,\"orderBy\":\"\",\"campus\":\"01\",\"SFCT\":\"0\"}";

    let json: serde_json::Value = serde_json::from_str(data)?;

    let json_body = loop {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("authorization", auth.parse()?);
        header.insert("batchid", "eb7b2a1a1a834276ab3594e9bc3f836b".parse()?);

        let response = reqwest::Client::new()
            .post(url)
            .headers(header)
            .json(&json)
            .send()
            .await?;

        let json_raw: serde_json::Value = response.json().await?;
        match json_raw["code"].as_str() {
            Some("401") => continue,
            _ => break json_raw,
        }
    };

    Ok(json_body)
}
