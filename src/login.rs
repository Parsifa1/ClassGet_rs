use crate::captcha::get_uuid_captcha;
use base64::{engine::general_purpose, Engine as _};
use soft_aes::aes::aes_enc_ecb;

fn encrypt(password: &str) -> anyhow::Result<String> {
    let plaintext = password.as_bytes();
    let key = b"MWMqg2tPcDkxcm11";
    let padding = Some("PKCS7");

    let encrypted = aes_enc_ecb(plaintext, key, padding).unwrap_or(vec![]);
    let vec_to_string = general_purpose::STANDARD.encode(encrypted);
    Ok(vec_to_string)
}

pub async fn log_in(acc: &str, password: &str) -> anyhow::Result<String> {
    let encrypt_password = encrypt(password)?;
    let url = "http://jwxk.hrbeu.edu.cn/xsxk/auth/hrbeu/login";

    let auth = loop {
        let (uuid, captcha) = get_uuid_captcha().await?;
        let payload = [
            ("loginname", acc),
            ("password", encrypt_password.as_str()),
            ("captcha", &captcha),
            ("uuid", &uuid),
        ];
        // println!("captcha: {}\nuuid: {}", captcha, uuid);
        let response = reqwest::Client::new()
            .post(url)
            .form(&payload)
            .send()
            .await?;

        let json_body: serde_json::Value = response.json().await?;
        match json_body["data"].as_str() {
            // Some("null") if json_body["msg"].as_str() == Some("未到选课开始时间") => {
            //     panic!("未到选课开始时间")
            // }
            Some("null") => continue,
            _ => {
                break json_body["data"]["token"]
                    .to_string()
                    .trim_matches('"')
                    .to_string()
            }
        };
    };
    Ok(auth)
}
