use teloxide::{prelude::{Requester, RequesterExt}, types::ParseMode, Bot};

use crate::{config::Config, fetch, stateful};


pub async fn daemon(config: &Config)
{
    let client = reqwest::Client::new();
    let bot = Bot::new(&config.telegram.bot_token).parse_mode(ParseMode::Markdown);
    loop {
        let messages = fetch::fetch_messages_details_for_all_channels(&client, &config).await;
        for message in messages {
            if stateful::process_stateful_message(&message.content, &message.channel_id) == true {
                println!("Message is new: {:?}", message);
                let formated_message = format!("**{}: {}**\n**At: {}**\n**By: {}**\n\n{}", message.guild_name, message.channel_name, message.timestamp, message.author_name, message.content);
                bot.send_message(config.telegram.chat_id.to_string(), &formated_message).await.unwrap();
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(config.general.trigger)).await;
    }
}