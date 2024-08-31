use std::hash::{Hash, Hasher};
use std::thread;
use fnv::FnvHasher;
use log::info;
use crate::engine::partition::Partition;
use crate::engine::{CacheItem, PartitionOperationError};

pub struct Database {
    pub(crate) partitions: Vec<Partition>,
    path_file: String,
    pub partition_number: u8,
}

impl Database {
    pub fn new(partition_number: u8, path_file: String) -> Database {
        let mut database = Database {
            partitions : Vec::new(),
            path_file,
            partition_number
        };
        let mut partition_resumed = 0;

        for i in 0..partition_number {
            let partition_path = format!("{}/partition_{}", database.path_file, i);
            database.partitions.push(Partition::new(i, partition_path));

            //try load data from file
            partition_resumed += database.partitions[i as usize].load_data();
        }

        info!("{} partitions resumed", partition_resumed);
        database
    }

    pub fn count_entries(&self) -> usize {
        self.partitions.iter().map(|partition| partition.count_entries()).sum()
    }

    pub fn delete_all(&mut self) {
        self.partitions.iter_mut().for_each(|partition| partition.delete_all());
    }

    pub fn keys(&self) -> Vec<String> {
        self.partitions.iter().map(|partition| partition.keys()).flatten().collect()
    }

    pub fn push(&mut self, key: String, value: Vec<u8>) -> Result<&CacheItem, PartitionOperationError> {
        let partition_index = self.get_partition_index(&key);
        self.partitions[partition_index].push(key, value)
    }

    pub fn pop(&mut self, key: String) -> Result<&CacheItem, PartitionOperationError> {
        let partition_index = self.get_partition_index(&key);
        self.partitions[partition_index].pop(key)
    }

    pub fn read(&self, key: String) -> Result<&CacheItem, PartitionOperationError> {
        let partition_index = self.get_partition_index(&key);
        self.partitions[partition_index].read(key)
    }

    pub fn persist(&self) {
        for partition in self.partitions.iter() {
            partition.persist();
        }
    }

    fn get_partition_index(&self, key: &String) -> usize {
        let mut hasher = FnvHasher::default();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        // Calcola il risultato modulo numero di partizioni
        let result = (hash % self.partition_number as u64) as usize;
        result
    }
}