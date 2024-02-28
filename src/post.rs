pub async fn print_all_class(data_json: &serde_json::Value) -> anyhow::Result<()> {
    let num = &data_json["data"]["total"].as_u64().unwrap_or(0);
    let num = *num as usize;
    for i in 1..num {
        println!(
            "{}: {} {}",
            i, data_json["data"]["rows"][i]["KCM"], data_json["data"]["rows"][i]["XGXKLB"]
        );
    }
    Ok(())
}

pub async fn get_class(
    num: usize,
    auth: String,
    data_json: serde_json::Value,
) -> anyhow::Result<()> {
    let pram = &data_json["data"]["rows"][num];

    let data = [
        ("clazzType", "XGKC"),
        ("clazzId", pram["JXBID"].as_str().unwrap_or("")),
        ("secretVal", pram["secretVal"].as_str().unwrap_or("")),
    ];

    let url = "***REMOVED***elective/clazz/add";
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

        if json_body["msg"] == "教学任务信息过期，请重新刷新列表" {
            return Err(anyhow::anyhow!(num.to_string()));
        }
        if json_body["msg"] != "请求过快，请登录后再试" {
            println!("{} {}", kcm, xgxklb);
            println!("{}", msg);
            // log::info!("{} {}", kcm, xgxklb);
            // log::info!("{}", msg);
        }

        if json_body["msg"] == "该课程已在选课结果中" {
            log::info!("{}", msg);
            log::info!("{} {}", kcm, xgxklb);
            break;
        }
    }

    Ok(())
}
