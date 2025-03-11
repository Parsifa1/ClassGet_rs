use std::sync::Arc;

use crate::params::{ClassError, FormatData, ValiPara};
use anyhow::{anyhow, bail};

pub async fn format_all_class(data: Arc<FormatData>) -> anyhow::Result<Vec<String>> {
    let data = data.as_ref();
    let (data_json, is_tjkc) = (&data.data_json, data.is_tjkc);
    let num = &data_json["data"]["total"].as_u64().unwrap_or(0);
    let arg_secend = if is_tjkc { "KCLB" } else { "XGXKLB" };
    let formatted_strings = (1..*num as usize)
        .map(|i| {
            format!(
                "{}: {}  {}",
                i, data_json["data"]["rows"][i]["KCM"], data_json["data"]["rows"][i][arg_secend]
            )
        })
        .collect::<Vec<_>>();
    if formatted_strings.is_empty() {
        return Err(anyhow::anyhow!("未能成功获取课程列表"));
    }
    Ok(formatted_strings)
}

pub async fn get_class(
    num: usize,
    urls: Arc<String>,
    classpara: Arc<ValiPara>,
    data: Arc<FormatData>,
) -> anyhow::Result<()> {
    let (data_json, is_tjkc) = (&data.data_json, data.is_tjkc);
    if num == 0 {
        return Ok(());
    }
    let pram = &data_json["data"]["rows"][num];

    let data = if is_tjkc {
        [
            ("clazzType", "TJKC"),
            ("clazzId", pram["tcList"][0]["JXBID"].as_str().unwrap_or("")),
            (
                "secretVal",
                pram["tcList"][0]["secretVal"].as_str().unwrap_or(""),
            ),
        ]
    } else {
        [
            ("clazzType", "XGKC"),
            ("clazzId", pram["JXBID"].as_str().unwrap_or("")),
            ("secretVal", pram["secretVal"].as_str().unwrap_or("")),
        ]
    };

    let url = urls.to_string() + "elective/clazz/add";
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
                .form(&data)
                .send()
                .await?;

            // 延时
            tokio::time::sleep(tokio::time::Duration::from_millis(350)).await;

            let json_body: serde_json::Value = response.json().await?;

            // log::info!("{}", json_body);

            let kcm = &data_json["data"]["rows"][num]["KCM"]
                .as_str()
                .ok_or(anyhow!("kcm转换失败"))?;
            let xgxklb = if is_tjkc {
                data_json["data"]["rows"][num]["KCLB"]
                    .as_str()
                    .ok_or(anyhow!("kclb转换失败"))?
            } else {
                data_json["data"]["rows"][num]["XGXKLB"]
                    .as_str()
                    .ok_or(anyhow!("kcm转换失败"))?
            };
            let msg = json_body["msg"].to_string();

            match json_body["msg"].as_str() {
                Some("参数校验不通过") | Some("教学任务信息过期，请重新刷新列表") =>
                {
                    let e = ClassError { value: num };
                    bail!(e);
                }
                Some("请求过快，请登录后再试") => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    bail!("请登录")
                }
                Some("该课程已在选课结果中") => {
                    log::info!("{} {} {} {}", "SUCSESS!", kcm, xgxklb, msg);
                    Ok(())
                }
                _ => {
                    log::info!("{} {} {}", kcm, xgxklb, msg);
                    bail!(msg);
                }
            }
        }
        .await
        {
            Ok(_) => {
                log::info!("{}课程选课成功", num);
                break;
            }
            Err(e) => {
                if e.to_string().contains("转换失败") {
                    log::warn!("{}", e);
                }
                continue;
            }
        }
    }

    Ok(())
}
