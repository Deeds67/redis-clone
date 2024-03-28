use dashmap::DashMap;

pub struct RedisDatabase {
    data: DashMap<String, String>,
}

impl RedisDatabase {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }

    pub fn set(&self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|value_ref| value_ref.clone())
    }

    pub fn del(&self, key: &str) -> Option<String> {
        self.data.remove(key).map(|(_, value)| value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let db = RedisDatabase::new();
        assert_eq!(db.get("any_key"), None);
    }

    #[test]
    fn test_set_and_get() {
        let db = RedisDatabase::new();
        db.set("key1".to_string(), "value1".to_string());
        assert_eq!(db.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_set_twice() {
        let db = RedisDatabase::new();
        db.set("key1".to_string(), "value1".to_string());
        assert_eq!(db.get("key1"), Some("value1".to_string()));
        db.set("key1".to_string(), "value2".to_string());
        assert_eq!(db.get("key1"), Some("value2".to_string()));
    }

    #[test]
    fn test_del() {
        let db = RedisDatabase::new();
        db.set("key1".to_string(), "value1".to_string());
        assert_eq!(db.del("key1"), Some("value1".to_string()));
        assert_eq!(db.get("key1"), None);
    }

    #[test]
    fn test_get_non_existent() {
        let db = RedisDatabase::new();
        assert_eq!(db.get("non_existent_key"), None);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;
        use std::sync::Arc;

        let db = Arc::new(RedisDatabase::new());
        let mut handles = vec![];

        for i in 0..100 {
            let db_clone = Arc::clone(&db);
            let handle = thread::spawn(move || {
                let key = format!("key{}", i);
                let value = format!("value{}", i);
                db_clone.set(key.clone(), value.clone());
                assert_eq!(db_clone.get(&key), Some(value));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        for i in 0..100 {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            assert_eq!(db.get(&key), Some(value));
        }
    }
}