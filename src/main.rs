mod captcha;
mod display;
mod login;
mod parser;
mod post;

use anyhow::Result;
use display::SpecializedDisplay;
use log::LevelFilter;
use login::{log_in, read_class};
use parser::get_data;
use post::print_all_class;
use serde_json::Value;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use tokio::task::JoinSet;

#[derive(Clone)]
pub struct ClassPara {
    pub auth: String,
    pub batchid: String,
}

async fn async_handler(class: Vec<usize>, classpara: ClassPara, data_json: Value) -> Result<()> {
    let mut set = JoinSet::new();
    class.iter().for_each(|&i| {
        set.spawn(post::get_class(i, classpara.clone(), data_json.clone()));
    });
    while let Some(res) = set.join_next().await {
        let Ok(task) = res else { continue };
        if let Err(e) = task {
            // let data_update = parser::get_data(&classpara).await?;
            set.spawn(post::get_class(
                match e.downcast_ref::<crate::post::ClassError>() {
                    Some(my_error) => my_error.value,
                    _ => 0,
                },
                classpara.clone(),
                data_json.clone(),
            ));
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConfigBuilder::new()
        .add_filter_allow("class_get".to_string())
        .build();

    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            config.clone(),
            File::create("get_class.log").unwrap(),
        ),
    ]);

    let para = log_in().await.display()?;
    let data_json = get_data(&para).await?;
    print_all_class(&data_json).await.display()?;

    println!("按回车键继续...");
    std::io::stdin().read_line(&mut String::new())?;

    let class = read_class().display()?;
    async_handler(class, para.clone(), data_json.clone()).await?;

    Ok(())
}
