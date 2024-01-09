mod captcha;
mod login;
mod parser;
mod post;

use tokio::task::JoinSet;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let account = "2022251124";
    let password = "13436968575@Ljp";

    let auth = login::log_in(&account, &password).await?;
    println!("{}", auth);
    let data_json = parser::get_data(&auth).await?;
    post::print_all_class(&data_json).await?;

    let class = vec![1, 2, 3];
    print!("你选择的课程为：");
    class.iter().for_each(|i| print!("{} ", i));
    let mut set = JoinSet::new();

    for i in class {
        set.spawn(post::get_class(i, auth.clone(), data_json.clone()));
    }
    while let Some(_) = set.join_next().await {}
    Ok(())
}
