mod captcha;
mod display;
mod login;
mod params;
mod parser;
mod post;

use crate::params::AsyncPara;
use crate::params::FormatData;
use anyhow::Result;
use display::SpecializedDisplay;
use log::{info, warn, LevelFilter};
use login::{log_in, read_account, read_class};
use parser::get_data;
use post::format_all_class;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use std::sync::Arc;
use tokio::task::JoinSet;

async fn async_handler(async_para: AsyncPara) -> Result<()> {
    let mut set = JoinSet::new();
    let (urls, class, classpara, data) = (
        async_para.urls,
        async_para.class,
        async_para.classpara,
        async_para.data,
    );
    class.iter().for_each(|&i| {
        set.spawn(post::get_class(
            i,
            urls.clone(),
            classpara.clone(),
            data.clone(),
        ));
    });
    while let Some(res) = set.join_next().await {
        warn!("当前同时工作选课协程数: {}", set.len());
        let Ok(task) = res else { continue };
        if let Err(e) = task {
            set.spawn(post::get_class(
                match e.downcast_ref::<crate::params::ClassError>() {
                    Some(my_error) => my_error.value,
                    _ => 1,
                },
                urls.clone(),
                classpara.clone(),
                data.clone(),
            ));
        }
    }
    Ok(())
}

fn logging() -> Result<()> {
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
            LevelFilter::Debug,
            config.clone(),
            File::create("get_class.log")?,
        ),
    ]);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    logging()?;

    let (urls, classtype) = read_account(true)?;
    let is_tjkc = classtype == "TJKC";
    info!("classtype: {}", classtype);

    let para = log_in(&urls).await.display()?;
    let data_json = get_data(&para, &urls, is_tjkc).await?;
    let data = Arc::new(FormatData { data_json, is_tjkc });
    format_all_class(data.clone()).await.display()?;

    println!("按回车键继续...");
    std::io::stdin().read_line(&mut String::new())?;

    let class = read_class().display()?;
    let para = AsyncPara {
        urls: Arc::new(urls),
        class: Arc::new(class),
        classpara: Arc::new(para),
        data,
    };
    async_handler(para).await?;

    Ok(())
}
