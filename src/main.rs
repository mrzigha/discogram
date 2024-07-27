#![allow(unused)] // silence unused while developing
use config::Config;

mod config;
mod fetch;

#[tokio::main]
async fn main()
{
    let config = Config::load();
    let client = reqwest::Client::new();
    let messages = fetch::fetch_messages_details_for_all_channels(&client, &config).await;
}
