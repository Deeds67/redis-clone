use std::collections::HashMap;

pub struct RedisDatabase {
    data: HashMap<String, String>,
}

impl RedisDatabase {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn del(&mut self, key: &str) -> Option<String> {
        return self.data.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let db = RedisDatabase::new();
        assert!(db.data.is_empty());
    }

    #[test]
    fn test_set() {
        let mut db = RedisDatabase::new();
        db.set("key1".to_string(), "value1".to_string());
        assert_eq!(db.data.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_get() {
        let mut db = RedisDatabase::new();
        db.set("key1".to_string(), "value1".to_string());
        assert_eq!(db.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_del() {
        let mut db = RedisDatabase::new();
        db.set("key1".to_string(), "value1".to_string());
        db.del("key1");
        assert_eq!(db.get("key1"), None);
    }

    #[test]
    fn test_del_nonexistent_key() {
        let mut db = RedisDatabase::new();
        let result = db.del("nonexistent_key");
        assert_eq!(result, None);
    }
}