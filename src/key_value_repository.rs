// use std::collections::HashMap;

// pub struct RedisDatabase {
//     data: HashMap<String, String>,
// }

// impl RedisDatabase {
//     pub fn new() -> Self {
//         Self {
//             data: HashMap::new(),
//         }
//     }

//     pub fn set(&mut self, key: String, value: String) {
//         self.data.insert(key, value);
//     }

//     pub fn get(&self, key: &str) -> Option<&String> {
//         self.data.get(key)
//     }

//     pub fn del(&mut self, key: &str) {
//         self.data.remove(key);
//     }
// }