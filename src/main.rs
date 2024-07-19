mod captcha;
mod display;
mod login;
mod parser;
mod post;

use display::SpecializedDisplay;
use login::{log_in, read_class};
use parser::get_data;
use post::print_all_class;
// use simplelog::{ConfigBuilder, LevelFilter};
use tokio::task::JoinSet;

async fn async_handler(
    class: Vec<usize>,
    auth: String,
    batchid: String,
    data_json: serde_json::Value,
) -> anyhow::Result<()> {
    let mut set = JoinSet::new();
    for i in class {
        set.spawn(post::get_class(
            i,
            auth.clone(),
            batchid.clone(),
            data_json.clone(),
        ));
    }
    while let Some(res) = set.join_next().await {
        if let Ok(task) = res {
            match task {
                Ok(_) => {}
                Err(e) => {
                    let data_update = parser::get_data(&auth, &batchid).await?;
                    set.spawn(post::get_class(
                        if let Some(my_error) = e.downcast_ref::<crate::post::ClassError>() {
                            my_error.value
                        } else {
                            0
                        },
                        auth.clone(),
                        batchid.clone(),
                        data_update.clone(),
                    ));
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (auth, batchid) = log_in().await.display()?;
    let data_json = get_data(&auth, &batchid).await?;
    print_all_class(&data_json).await.display()?;

    println!("按回车键继续...");
    std::io::stdin().read_line(&mut String::new())?;

    let class = read_class().display()?;
    async_handler(class, auth.clone(), batchid, data_json.clone()).await?;

    Ok(())
}
