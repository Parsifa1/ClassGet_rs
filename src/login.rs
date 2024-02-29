use crate::captcha::get_uuid_captcha;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use soft_aes::aes::aes_enc_ecb;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub account: String,
    pub password: String,
    pub class: Vec<usize>,
}

pub fn read_class() -> Vec<usize> {
    let config_in = std::fs::read_to_string("config.yaml");
    let config = match config_in {
        Ok(config) => config,
        Err(_) => match std::fs::read_to_string("../config.yaml") {
            Ok(config) => config,
            Err(_) => {
                let mut files =
                    std::fs::File::create("config.yaml").expect("自动创建空配置文件夹失败");
                let bytes: &[u8] =
                    b"account: 114514\npassword: 1919810\nclass: [1, 1, 4, 5, 1, 4]";
                files.write_all(bytes).expect("自动写入配置文件夹失败");
                String::new()
            }
        },
    };
    let user_config: Config =
        serde_yaml::from_str(&config).expect("配置文件读取失败，请检查是否存在配置文件，若不存在，将会自动创建");
    user_config.class
}

fn read_account() -> anyhow::Result<(String, String)> {
    let config_in = std::fs::read_to_string("config.yaml");
    let config = match config_in {
        Ok(config) => config,
        Err(_) => match std::fs::read_to_string("../config.yaml") {
            Ok(config) => config,
            Err(_) => {
                let mut files =
                    std::fs::File::create("config.yaml").expect("自动创建空配置文件夹失败");
                let bytes: &[u8] =
                    b"account: 114514\npassword: 1919810\nclass: [1, 1, 4, 5, 1, 4]";
                files.write_all(bytes).expect("自动写入配置文件夹失败");
                String::new()
            }
        },
    };
    let user_config: Config =
        serde_yaml::from_str(&config).expect("配置文件读取失败，请检查是否存在配置文件，若不存在，将会自动创建");
    Ok((user_config.account, user_config.password))
}

fn encrypt(password: &str) -> anyhow::Result<String> {
    let plaintext = password.as_bytes();
    let key = b"MWMqg2tPcDkxcm11";
    let padding = Some("PKCS7");

    let encrypted = aes_enc_ecb(plaintext, key, padding).unwrap_or_default();
    let vec_to_string = general_purpose::STANDARD.encode(encrypted);
    Ok(vec_to_string)
}

pub async fn log_in() -> anyhow::Result<String> {
    let (acc, password) = read_account()?;
    let acc = &acc;
    let encrypt_password = encrypt(&password)?;
    let url = "***REMOVED***auth/hrbeu/login";

    let auth = loop {
        let (uuid, captcha) = get_uuid_captcha().await?;
        let payload = [
            ("loginname", acc),
            ("password", &encrypt_password),
            ("captcha", &captcha),
            ("uuid", &uuid),
        ];
        let response = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
            .post(url)
            .form(&payload)
            .send()
            .await?;

        let json_body: serde_json::Value = response.json().await?;
        match json_body["data"].as_str() {
            Some("管理员变更数据或账号在其他地方登录，请重新登录") => {
                continue
            }
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
