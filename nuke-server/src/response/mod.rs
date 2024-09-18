use serde::{Deserialize, Serialize};

pub trait CommandResponse {
    fn to_json(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueResponse {
    pub(crate) key: String,
    pub(crate) value: Vec<u8>,
}

impl CommandResponse for ValueResponse {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysResponse {
    pub(crate) keys: Vec<String>,
}

impl CommandResponse for KeysResponse {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionsDetailsResponse {
    pub(crate) partitions: Vec<PartitionDetail>,
}

impl CommandResponse for PartitionsDetailsResponse {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionDetail {
    pub(crate) partition: u8,
    pub(crate) keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminCommandResponse {
    message: String,
    ok: bool,
}

impl CommandResponse for AdminCommandResponse {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u8,
    pub message: String,
}

impl CommandResponse for ErrorResponse {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}