use serde_derive::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read,Write};

use sha3::Sha3_256;
use digest::Digest;

#[derive(Serialize, Deserialize, Clone)]
struct MessageObject {
    channel_id: String,
    hash: String,
}

fn save_message_hash(newhash: &MessageObject)
{
    const STATE_FILE: &str = ".discogram/state.json";
    let mut file_content = String::new();

    match OpenOptions::new().read(true).open(STATE_FILE) {
        Ok(mut file) => {
            match file.read_to_string(&mut file_content) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Failed to read file.");
                    return;
                }
            }
        }
        Err(_) => {
            match OpenOptions::new().create(true).write(true).open(STATE_FILE) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Failed to create file.");
                    return;
                }
            }
        }
    };

    let mut messages: Vec<MessageObject> = if !file_content.is_empty() {
        match serde_json::from_str(&file_content) {
            Ok(json) => json,
            Err(_) => {
                eprintln!("Failed to parse JSON data.");
                return;
            }
        }
    } else {
        Vec::new()
    };

    let mut found = false;
    for message in messages.iter_mut() {
        if message.channel_id == newhash.channel_id {
            message.hash = newhash.hash.clone();
            found = true;
            break;
        }
    }

    if !found {
        messages.push(newhash.clone());
    }

    let json = serde_json::to_string(&messages).unwrap();
    match OpenOptions::new().write(true).open(STATE_FILE) {
        Ok(mut file) => {
            match file.write_all(json.as_bytes()) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Failed to write to file.");
                    return;
                }
            }
        }
        Err(_) => {
            eprintln!("Failed to open file.");
            return;
        }
    };
}

fn hash_compare(newhash: &MessageObject) -> bool
{
    const STATE_FILE: &str = ".discogram/state.json";
    let mut file_content = String::new();

    match OpenOptions::new().read(true).open(STATE_FILE) {
        Ok(mut file) => {
            match file.read_to_string(&mut file_content) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Failed to read file.");
                    return false;
                }
            }
        }
        Err(_) => {
            return false;
        }
    };

    let messages: Vec<MessageObject> = if !file_content.is_empty() {
        match serde_json::from_str(&file_content) {
            Ok(json) => json,
            Err(_) => {
                eprintln!("Failed to parse JSON data.");
                return false;
            }
        }
    } else {
        return false;
    };

    for message in messages {
        if message.channel_id == newhash.channel_id {
            return message.hash == newhash.hash;
        }
    }
    return false;
}


fn hash_message(message: &str, channel_id: &str) -> MessageObject
{
    let mut hasher = Sha3_256::new();
    hasher.update(message.as_bytes());
    let result = hasher.finalize();
    let result_str = hex::encode(result);
    let result_obj = MessageObject {
        channel_id: channel_id.to_string(),
        hash: result_str,
    };
    return result_obj;
}

pub fn process_stateful_message(message: &str, channel_id: &str) -> bool
{
    let hash = hash_message(message, channel_id);
    if hash_compare(&hash) {
        return false;
    } else {
        save_message_hash(&hash);
        return true;
    }
}
