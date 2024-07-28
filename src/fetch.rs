use serde_derive::Deserialize;

use crate::config;

#[derive(Deserialize, Debug)]
struct RawMessage {
    id: String,
    content: String,
    author: Author,
    timestamp: String,
    channel_id: String,
}

#[derive(Deserialize, Debug)]
struct Author {
    username: String,
}

#[derive(Deserialize, Debug)]
struct Channel {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Guild {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct MessageDetails {
    pub channel_name: String,
    pub channel_id: String,
    pub author_name: String,
    pub timestamp: String,
    pub content: String,
    pub guild_name: String,
}

pub async fn fetch_messages_details_for_all_channels(client: &reqwest::Client, config: &config::Config) -> Vec<MessageDetails>
{
    let mut message_details = Vec::new();
    const BASE_URL: &str = "https://discord.com/api/v10";

    for channel in &config.channels {
        let messages_url = format!("{}/channels/{}/messages", BASE_URL, channel.id);
        let channel_url = format!("{}/channels/{}", BASE_URL, channel.id);

        match fetch_messages_details(client, channel.name.clone(),&messages_url, &channel_url, &config.discord.auth_token).await {
            Ok(mut messages) => {
                message_details.append(&mut messages);
            }
            Err(e) => {
                eprintln!("Failed to fetch messages for channel {}/{}: {:?}", channel.name, channel.id, e);
            }
        }
    }

    return message_details;
}

async fn fetch_messages_details(client: &reqwest::Client, guild_name: String, messages_url: &str, channel_url: &str, token: &str) -> Result<Vec<MessageDetails>, reqwest::Error>
{
    let messages = match fetch_messages(client, messages_url, token).await {
        Ok(messages) => messages,
        Err(e) => {
            return Err(e);
        }
    };

    let channel_name = match fetch_channel_name(client, channel_url, token).await {
        Ok(channel_name) => channel_name,
        Err(e) => {
            return Err(e);
        }
    };

    let message_details: Vec<MessageDetails> = messages
        .into_iter()
        .map(|msg| MessageDetails {
            channel_name: channel_name.clone(),
            channel_id: msg.channel_id,
            author_name: msg.author.username,
            timestamp: msg.timestamp,
            content: msg.content,
            guild_name: guild_name.clone(),
        })
        .collect();

    return Ok(message_details);
}

async fn fetch_messages(client: &reqwest::Client, url: &str, token: &str) -> Result<Vec<RawMessage>, reqwest::Error>
{
    let res = match client.get(url).header("Authorization", token).send().await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to fetch messages: {:?}", e);
            return Err(e);
        }
    };

    let messages: Vec<RawMessage> = match res.json().await {
        Ok(messages) => messages,
        Err(e) => {
            eprintln!("Failed to parse messages: {:?}", e);
            return Err(e);
        }
    };

    return Ok(messages.into_iter().take(1).collect());
}

async fn fetch_channel_name(client: &reqwest::Client, url: &str, token: &str) -> Result<String, reqwest::Error>
{
    let res = match client.get(url).header("Authorization", token).send().await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to fetch channel: {:?}", e);
            return Err(e);
        }
    };

    let channel: Channel = match res.json().await  {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to parse channel: {:?}", e);
            return Err(e);
        }
    };

    return Ok(channel.name);
}

async fn fetch_guild_name(client: &reqwest::Client, url: &str, token: &str) -> Result<String, reqwest::Error> 
{
    let res = match client.get(url).header("Authorization", token).send().await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to fetch guild: {:?}", e);
            return Err(e);
        }
    };

    let guild: Guild = match res.json().await {
        Ok(guild) => guild,
        Err(e) => {
            eprintln!("Failed to parse guild: {:?}", e);
            return Err(e);
        }
    };

    return Ok(guild.name);
}
