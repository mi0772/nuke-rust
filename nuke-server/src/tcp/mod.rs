use std::sync::{Arc, Mutex};
use crate::engine::database::Database;
use crate::response::{CommandResponse, ErrorResponse, KeysResponse, PartitionDetail, PartitionsDetailsResponse, ValueResponse};
use crate::tcp::Command::PartitionsDetails;

#[derive(Debug)]
pub(crate) enum Command {
    Pop(String),
    Push(String, Vec<u8>),
    Read(String),
    Keys,
    PartitionsDetails,
    Clear,
    Quit,
}

impl Command {
    pub(crate) fn from_str(command: &str) -> Result<Command, String> {
        let command = command.to_ascii_lowercase();
        let mut parts = command.split_whitespace();
        match parts.next() {
            Some("pop") => {
                match parts.next() {
                    Some(key) => Ok(Command::Pop(key.to_string())),
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
            Some("partitions_details") => Ok(Command::PartitionsDetails),
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
        Command::Pop(key) => {
            match db.pop(key.to_string()) {
                Ok(cache_item) => ValueResponse{key, value: cache_item.value.clone()}.to_json(),
                Err(_) => ErrorResponse{code: 0, message:"Key not found".to_string()}.to_json(),
            }
        }
        Command::Push(key, value) => {
            match db.push(key.to_string(), value.to_vec()) {
                Ok(_) => ValueResponse{key, value: value.clone()}.to_json(),
                Err(_) => ErrorResponse{code: 0, message:"Error pushing value".to_string()}.to_json(),
            }
        }
        Command::Read(key) => {
            match db.read(key.to_string()) {
                Ok(cache_item) => ValueResponse{key, value: cache_item.value.clone()}.to_json(),
                Err(e) => ErrorResponse{code: 0, message:"Key not found".to_string()}.to_json(),
            }
        }
        Command::Keys => {
            KeysResponse{keys: db.keys()}.to_json()
        }
        Command::PartitionsDetails => {
            let mut partitions_details = Vec::new();
            for partition in db.partitions.iter() {
                let pd = PartitionDetail{partition: partition.partition_number, keys: partition.keys()};
                partitions_details.push(pd);
            }
            PartitionsDetailsResponse{ partitions: partitions_details}.to_json()
        }
        Command::Clear => {
            db.delete_all();
            ErrorResponse{code: 0, message:"Database cleared".to_string()}.to_json()
        }
        Command::Quit => {
            ErrorResponse{code: 0, message:"Quitting".to_string()}.to_json()
        }
    }
}