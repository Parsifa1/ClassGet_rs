mod captcha;
mod login;
mod parser;
mod post;

use crate::login::Config;
use simplelog::{ConfigBuilder, LevelFilter};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logconfig = ConfigBuilder::new()
        .add_filter_allow_str("crate::login")
        .build();

    let _ = simplelog::WriteLogger::init(
        LevelFilter::Info,
        logconfig,
        std::fs::File::create("get_class.log")?,
    );

    let auth = login::log_in().await?;
    let data_json = parser::get_data(&auth).await?;
    post::print_all_class(&data_json).await?;

    println!("按回车键继续...");
    std::io::stdin().read_line(&mut String::new())?;
    let class = read_class();

    println!("你选择的课程为：");
    class.iter().for_each(|i| print!("{} ", i));

    let mut set = JoinSet::new();
    for i in class {
        set.spawn(post::get_class(i, auth.clone(), data_json.clone()));
    }
    while let Some(res) = set.join_next().await {
        match res {
            Ok(task) => {
                match task {
                    Ok(_) => {}
                    Err(e) => {
                        let data_update = parser::get_data(&auth).await?;
                        set.spawn(post::get_class(e.to_string().parse::<usize>().unwrap(), auth.clone(), data_update.clone()));

                    }
                }
            }
            Err(_) => {}
        }
    }
    Ok(())
}

fn read_class() -> Vec<usize> {
    let config_in = std::fs::read_to_string("config.yaml");
    let config = match config_in {
        Ok(config) => config,
        Err(_) => std::fs::read_to_string("../config.yaml").expect("config.yaml read failed!"),
    };
    let user_config: Config = serde_yaml::from_str(&config).expect("config.yaml parse failed!");
    user_config.class
}
