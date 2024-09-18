use crate::engine::{key_hasher, CacheItem, PartitionOperationError};
use serde_json::to_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Partition {
    entries: HashMap<String, CacheItem>,
    pub(crate) partition_number: u8,
    partition_path: String,
    mutex: Arc<RwLock<u8>>,
    persisted: bool,
}

impl Partition {
    //function that try to load serialized content of partition from file
    pub(in crate::engine) fn load_data(&mut self) -> i32 {
        let file = match File::open(&self.partition_path) {
            Ok(file) => file,
            Err(_) => {
                return 0;
            }
        };

        // Deserialize the JSON
        let entries: HashMap<String, CacheItem> = match serde_json::from_reader(file) {
            Ok(entries) => entries,
            Err(err) => {
                panic!("Errore nel deserializzare il file: {}", err);
            }
        };

        // Update the entries
        self.entries = entries;
        self.persisted = true;
        1
    }
}

impl Partition {
    pub(in crate::engine) fn new(partition_number: u8, partition_path: String) -> Partition {
        Partition {
            entries: HashMap::new(),
            partition_number,
            partition_path,
            mutex: Arc::new(RwLock::new(0)),
            persisted: false,
        }
    }

    pub(in crate::engine) fn push(&mut self, key: String, value: Vec<u8>) -> Result<&CacheItem, PartitionOperationError> {
        let mut mutex = self.mutex.write().unwrap();
        let cache_item = CacheItem {
            key: key.clone(),
            hashed_key: key_hasher(&key),
            value,
            deleted: false,
        };
        self.entries.insert(key.clone(), cache_item);
        *mutex = 0;
        Ok(self.entries.get(&key).unwrap())
    }

    pub(in crate::engine) fn pop(&mut self, key: String) -> Result<&CacheItem, PartitionOperationError> {
        let mut mutex = self.mutex.write().unwrap();
        let cache_item = self.entries.get_mut(&key).unwrap();
        if !cache_item.deleted {
            cache_item.deleted = true;
            *mutex = 0;
            Ok(self.entries.get(&key).unwrap())
        } else {
            *mutex = 0;
            Err(PartitionOperationError::CacheItemNotFound)
        }
    }

    pub(in crate::engine) fn read(&self, key: String) -> Result<&CacheItem, PartitionOperationError> {
        let _mutex = self.mutex.read().unwrap();
        let cache_item = self.entries.get(&key);
        if let Some(item) = cache_item {
            if item.deleted {
                return Err(PartitionOperationError::ReadError);
            }
            Ok(item)
        } else {
            Err(PartitionOperationError::CacheItemNotFound)
        }

    }

    pub(in crate::engine) fn persist(&self) {
        // Acquisisce il lock in scrittura
        let _write_lock = self.mutex.write().unwrap();

        // Serializza entries in JSON
        let json = match to_string(&self.entries) {
            Ok(json) => json,
            Err(err) => {
                eprintln!("cannot marshal partition data: {}", err);
                return;
            }
        };

        // Crea il file
        let file = match File::create(&self.partition_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Errore nella creazione del file: {}", err);
                return;
            }
        };

        // Scrive il JSON nel file
        let mut writer = BufWriter::new(file);
        if let Err(err) = writer.write_all(json.as_bytes()) {
            eprintln!("Errore nella scrittura con il buffer: {}", err);
            return;
        }

        // Flush del buffer
        if let Err(err) = writer.flush() {
            eprintln!("Errore nel flush dei dati al file: {}", err);
        }
    }

    pub(in crate::engine) fn is_persisted(&self) -> bool {
        self.persisted
    }

    pub(crate) fn count_entries(&self) -> usize {
        self.entries.len()
    }

    pub(in crate::engine) fn delete_all(&mut self) {
        self.entries.clear();
    }

    pub(crate) fn keys(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }
}