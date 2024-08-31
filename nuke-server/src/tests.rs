#[cfg(test)]
mod tests {
    use crate::engine::database::Database;

    #[test]
    fn test_new_database() {
        let database = Database::new(10, "test".to_string());
        assert_eq!(database.count_entries(), 10);
    }

    #[test]
    fn test_count_entries() {
        let mut database = Database::new(10, "test".to_string());
        database.push("key".to_string(), vec![1, 2, 3]);
        assert_eq!(database.count_entries(), 1);
    }

    #[test]
    fn test_push_1000_entries() {
        let mut database = Database::new(10, "test".to_string());
        for i in 0..1000 {
            database.push(format!("key_{}", i), vec![1, 2, 3]);
        }
        assert_eq!(database.count_entries(), 1000);
    }
}
