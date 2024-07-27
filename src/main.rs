#![allow(unused)] // silence unused while developing
use config::Config;
use fetch::fetch_messages_details_for_all_channels;

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
