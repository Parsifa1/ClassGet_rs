mod captcha;
mod login;
mod parser;
mod post;

use simplelog::{ConfigBuilder, LevelFilter};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logconfig = ConfigBuilder::new().build();
    let _ = simplelog::WriteLogger::init(
        LevelFilter::Info,
        logconfig,
        std::fs::File::create("get_class.log")?,
    );
    let auth = login::log_in().await?;
    let data_json = parser::get_data(&auth).await?;

    match post::print_all_class(&data_json).await {
        Ok(class_list) => {
            println!("你的课程列表为：");
            class_list.iter().for_each(|i| println!("{} ", i));
            println!();
        }
        Err(e) => {
            println!("{}, 请重启程序", e);
            return Ok(());
        }
    }
    println!("按回车键继续...");
    std::io::stdin().read_line(&mut String::new())?;
    let class = login::read_class();
    println!("你选择的课程为：");
    class.iter().for_each(|i| print!("{} ", i));
    println!();

    let mut set = JoinSet::new();
    for i in class {
        set.spawn(post::get_class(i, auth.clone(), data_json.clone()));
    }
    while let Some(res) = set.join_next().await {
        if let Ok(task) = res {
            match task {
                Ok(_) => {}
                Err(e) => {
                    let data_update = parser::get_data(&auth).await?;
                    set.spawn(post::get_class(
                        if let Some(my_error) = e.downcast_ref::<crate::post::ClassError>() {
                            my_error.value
                        } else {
                            0
                        },
                        auth.clone(),
                        data_update.clone(),
                    ));
                }
            }
        }
    }
    Ok(())
}
