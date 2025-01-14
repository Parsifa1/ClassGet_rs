use anyhow::anyhow;
use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use image::ImageOutputFormat::Png;
use log::info;
use std::io::Cursor;

fn save_image_from_data_uri(data_uri: Option<&str>) -> anyhow::Result<DynamicImage> {
    let parse_data_uri = |uri: Option<&str>| -> Option<(String, String)> {
        let uri = match uri {
            Some(uri) => uri,
            None => return None,
        };
        if let Some(pos) = uri.find(',') {
            let (mime, data) = uri.split_at(pos + 1);
            let mime = mime.trim_end_matches(',');
            Some((mime.to_string(), data.to_string()))
        } else {
            None
        }
    };

    if let Some((mime, data)) = parse_data_uri(data_uri) {
        if mime == "data:image/png;base64" {
            let decoded_data = general_purpose::STANDARD.decode(data)?;
            let image = image::load_from_memory(&decoded_data)?;
            Ok(image)
        } else {
            Err(anyhow!("解析验证码图片失败"))
        }
    } else {
        Err(anyhow!("获取验证码图片失败"))
    }
}

fn ocr_image(image: DynamicImage) -> anyhow::Result<String> {
    let mut ocr = ddddocr::ddddocr_classification().expect("dddd失败");
    let mut img_raw = Cursor::new(Vec::new());
    image.write_to(&mut img_raw, Png)?;
    ocr.classification(img_raw.into_inner(), false)
}

pub async fn get_uuid_captcha(urls: &String) -> anyhow::Result<(String, String)> {
    let url = urls.to_string() + "auth/captcha";

    loop {
        match async {
            let response = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap()
                .post(&url)
                .send()
                .await?;
            let json_body: serde_json::Value = response.json().await?;
            let capt_name = json_body["data"]["captcha"].as_str();
            let capt_uuid = json_body["data"]["uuid"].as_str();

            let image = save_image_from_data_uri(capt_name)?;
            let captcha = ocr_image(image)?;

            capt_uuid
                .map(|uuid| (uuid.to_string(), captcha))
                .ok_or(anyhow::anyhow!("Failed to get captcha"))
        }
        .await
        {
            Ok(result) => break Ok(result),
            Err(e) => {
                info!("获取验证码失败: {}，正在重试...", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
        }
    }
}
