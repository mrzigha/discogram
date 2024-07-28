use config::Config;

mod config;
mod stateful;
mod fetch;
mod daemon;

#[tokio::main]
async fn main()
{
    let config = Config::load();
    daemon::daemon(&config).await;
}
