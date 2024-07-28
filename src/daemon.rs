use crate::{config::Config, fetch, stateful};


pub async fn daemon(config: &Config)
{
    let client = reqwest::Client::new();
    loop {
        let messages = fetch::fetch_messages_details_for_all_channels(&client, &config).await;
        for message in messages {
            if stateful::process_stateful_message(&message.content, &message.channel_id) == true {
                println!("Message is new: {:?}", message);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(config.general.trigger)).await;
    }
}