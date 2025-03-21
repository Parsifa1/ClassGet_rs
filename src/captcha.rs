use base64::{engine::general_purpose, Engine as _};

fn save_image_from_data_uri(data_uri: Option<&str>) -> anyhow::Result<()> {
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
            image.save("output.png")?;
        }
    }
    Ok(())
}

fn ocr_image() -> anyhow::Result<String> {
    let image = std::fs::read("output.png")?;
    let mut ocr = ddddocr::ddddocr_classification()?;
    let res = ocr.classification(image);
    std::fs::remove_file("output.png")?;
    res
}

pub async fn get_uuid_captcha() -> anyhow::Result<(String, String)> {
    let url = "***REMOVED***auth/captcha";

    let response = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(url)
        .send()
        .await?;
    let json_body: serde_json::Value = response.json().await?;

    let capt_name = json_body["data"]["captcha"].as_str();
    let capt_uuid = json_body["data"]["uuid"].as_str();

    save_image_from_data_uri(capt_name)?;

    let captcha = ocr_image()?;

    capt_uuid
        .map(|uuid| (uuid.to_string(), captcha))
        .ok_or(anyhow::anyhow!("Failed to get captcha"))
}
