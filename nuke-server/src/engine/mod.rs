use std::hash::{Hash, Hasher};
use fnv::FnvHasher;
use serde::{Deserialize, Serialize};

pub(crate) mod database;
pub(crate) mod partition;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CacheItem {
    pub key: String,
    pub hashed_key: u64,
    pub value: Vec<u8>,
    pub deleted: bool,
}

// definie error for insert, delete and get for partitions
#[derive(Debug)]
pub enum PartitionOperationError {
    PushError,
    PopError,
    ReadError,
    CacheItemNotFound,
}

pub fn key_hasher(key: &String) -> u64 {
    let mut hasher = FnvHasher::default();
    key.hash(&mut hasher);
    hasher.finish()
}

