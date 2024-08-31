use std::ascii::AsciiExt;
use std::sync::{Arc, Mutex};
use crate::engine::database::Database;

#[derive(Debug)]
pub(crate) enum Command {
    Get(String),
    Push(String, Vec<u8>),
    Read(String),
    Keys,
    Partitions_details,
    Clear,
    Quit,
}

impl Command {
    pub(crate) fn from_str(command: &str) -> Result<Command, String> {
        let command = command.to_ascii_lowercase();
        let mut parts = command.split_whitespace();
        match parts.next() {
            Some("get") => {
                match parts.next() {
                    Some(key) => Ok(Command::Get(key.to_string())),
                    None => Err("Missing key".to_string()),
                }
            }
            Some("push") => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("Missing key".to_string()),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("Missing value".to_string()),
                };
                Ok(Command::Push(key.to_string(), value.as_bytes().to_vec()))
            }
            Some("read") => {
                match parts.next() {
                    Some(key) => Ok(Command::Read(key.to_string())),
                    None => Err("Missing key".to_string()),
                }
            }
            Some("keys") => Ok(Command::Keys),
            Some("partitions_details") => Ok(Command::Partitions_details),
            Some("clear") => Ok(Command::Clear),
            Some("quit") => Ok(Command::Quit),
            Some(command) => Err(format!("Unrecognized command: {}", command)),
            None => Err("Empty command".to_string()),
        }
    }
}

pub(crate) async fn handle_request(command: Command, database: &Arc<Mutex<Database>>) -> String {
    let mut db = database.lock().unwrap();
    match command {
        Command::Get(key) => {
            match db.read(key.to_string()) {
                Ok(cache_item) => format!("Value: {:?}", cache_item.value),
                Err(_) => "Key not found".to_string(),
            }
        }
        Command::Push(key, value) => {
            match db.push(key.to_string(), value.to_vec()) {
                Ok(_) => "Value pushed".to_string(),
                Err(_) => "Error pushing value".to_string(),
            }
        }
        Command::Read(key) => {
            match db.read(key.to_string()) {
                Ok(cache_item) => format!("Value: {:?}", cache_item.value),
                Err(e) => format!("Key not found : {:?}", e).to_string(),
            }
        }
        Command::Keys => {
            format!("Keys: {:?}", db.keys())
        }
        Command::Partitions_details => {
            let mut partitions_details = Vec::new();
            for partition in db.partitions.iter() {
                partitions_details.push(format!("Partition {}: {}", partition.partition_number, partition.count_entries()));
            }
            partitions_details.join("\n")
        }
        Command::Clear => {
            db.delete_all();
            "Database cleared".to_string()
        }
        Command::Quit => {
            "Quitting".to_string()
        }
    }
}