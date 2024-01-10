use serde_json;

pub async fn print_all_class(data_json: &serde_json::Value) -> anyhow::Result<()> {
    let num = &data_json["data"]["total"].as_u64().unwrap_or(0);
    let num = *num as usize;
    for i in 1..num {
        println!("{}: {}", i, data_json["data"]["rows"][i]["KCM"]);
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

    let url = "http://jwxk.hrbeu.edu.cn/xsxk/elective/clazz/add";
    loop {
        let mut header = reqwest::header::HeaderMap::new();
        header.insert("authorization", auth.parse()?);
        header.insert("batchid", "eb7b2a1a1a834276ab3594e9bc3f836b".parse()?);
        let response = reqwest::Client::new()
            .post(url)
            .headers(header)
            .form(&data)
            .send()
            .await?;

        let json_body: serde_json::Value = response.json().await?;
        let kcm = &data_json["data"]["rows"][num]["KCM"];
        let xgxklb = &data_json["data"]["rows"][num]["XGXKLB"];
        let msg = &json_body["msg"];
        if json_body["msg"] != "请求过快，请登录后再试" {
            println!("{} {}", kcm, xgxklb);
            println!("{}", msg);
            log::info!("{} {}", kcm, xgxklb);
            log::info!("{}", msg);
        }

        if json_body["msg"] == "该课程已在选课结果中" {
            log::info!("{}", msg);
            log::info!("{} {}", kcm, xgxklb);
            break;
        }
    }

    Ok(())
}
