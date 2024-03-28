use crate::{key_value_repository::RedisDatabase, resp::RespType};

pub struct RedisActionHandler {
    db: RedisDatabase,
}

impl RedisActionHandler {
    pub fn new(db: RedisDatabase) -> Self {
        RedisActionHandler { db }
    }
    
    pub fn handle(&mut self, resp: RespType) -> RespType {
        let action = RedisAction::from(resp);
        match action {
            RedisAction::Set(key, value) => {
                self.db.set(key, value);
                RespType::SimpleString("OK".to_string())
            }
            RedisAction::Get(key) => {
                if let Some(value) = self.db.get(&key) {
                    RespType::BulkString(value.as_bytes().to_vec())
                } else {
                    RespType::Null
                }
            }
            RedisAction::Del(key) => {
                self.db.del(&key);
                RespType::SimpleString("OK".to_string())
            }
            RedisAction::Ping() => RespType::SimpleString("PONG".to_string()),
            RedisAction::Unknown() => RespType::Error("Unknown command".to_string()),
        }
    }
    
}

#[derive(Debug, PartialEq)]
pub enum RedisAction {
    Set(String, String),
    Get(String),
    Del(String),
    Ping(),
    Unknown(),
}

impl From<RespType> for RedisAction {
    fn from(resp: RespType) -> Self {
        fn command_to_action(command: String) -> RedisAction {
            let parts: Vec<&str> = command.split_whitespace().collect();
            match parts.as_slice() {
                ["SET", key, value] => RedisAction::Set(key.to_string(), value.to_string()),
                ["GET", key] => RedisAction::Get(key.to_string()),
                ["DEL", key] => RedisAction::Del(key.to_string()),
                ["PING"] => RedisAction::Ping(),
                _ => RedisAction::Unknown(),
            }
        }

        match resp {
            RespType::SimpleString(command) => command_to_action(command),
            RespType::Array(array) => {
                let commands: Vec<String> = array.iter().filter_map(|item| {
                    if let RespType::BulkString(command) = item {
                        Some(String::from_utf8_lossy(command).to_string())
                    } else {
                        None
                    }
                }).collect();

                if !commands.is_empty() {
                    command_to_action(commands.join(" "))
                } else {
                    RedisAction::Unknown()
                }
            }
            _ => RedisAction::Unknown(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_resp_type_simple_string() {
        let resp = RespType::SimpleString(String::from("SET key value"));
        let action = RedisAction::from(resp);
        assert_eq!(
            action,
            RedisAction::Set(String::from("key"), String::from("value"))
        );
    }

    #[test]
    fn test_from_resp_type_array_with_bulk_string() {
        let resp = RespType::Array(vec![
            RespType::BulkString(b"GET".to_vec()),
            RespType::BulkString(b"key".to_vec()),
        ]);
        let action = RedisAction::from(resp);
        assert_eq!(action, RedisAction::Get(String::from("key")));
    }

    #[test]
    fn test_from_resp_type_array_without_bulk_string() {
        let resp = RespType::Array(vec![
            RespType::Integer(123),
            RespType::SimpleString(String::from("unknown")),
        ]);
        let action = RedisAction::from(resp);
        assert_eq!(action, RedisAction::Unknown());
    }

    #[test]
    fn test_handle_set() {
        let mut handler = RedisActionHandler::new(RedisDatabase::new());
        let resp = handler.handle(RespType::SimpleString("SET key value".to_string()));
        assert_eq!(resp, RespType::SimpleString("OK".to_string()));
    }

    #[test]
    fn test_handle_set_same_key_twice() {
        let mut handler = RedisActionHandler::new(RedisDatabase::new());
        let resp = handler.handle(RespType::SimpleString("SET key value".to_string()));
        assert_eq!(resp, RespType::SimpleString("OK".to_string()));
        let resp = handler.handle(RespType::SimpleString("GET key".to_string()));
        assert_eq!(resp, RespType::BulkString(b"value".to_vec()));
        let resp = handler.handle(RespType::SimpleString("SET key other_value".to_string()));
        assert_eq!(resp, RespType::SimpleString("OK".to_string()));
        let resp = handler.handle(RespType::SimpleString("GET key".to_string()));
        assert_eq!(resp, RespType::BulkString(b"other_value".to_vec()));
    }

    #[test]
    fn test_handle_get() {
        let mut handler = RedisActionHandler::new(RedisDatabase::new());
        handler.handle(RespType::SimpleString("SET key value".to_string()));
        let resp = handler.handle(RespType::SimpleString("GET key".to_string()));
        assert_eq!(resp, RespType::BulkString(b"value".to_vec()));
    }

    #[test]
    fn test_handle_get_nonexistent() {
        let mut handler = RedisActionHandler::new(RedisDatabase::new());
        let resp = handler.handle(RespType::SimpleString("GET key".to_string()));
        assert_eq!(resp, RespType::Null);
    }

    #[test]
    fn test_handle_del() {
        let mut handler = RedisActionHandler::new(RedisDatabase::new());
        handler.handle(RespType::SimpleString("SET key value".to_string()));
        let resp = handler.handle(RespType::SimpleString("DEL key".to_string()));
        assert_eq!(resp, RespType::SimpleString("OK".to_string()));
    }

    #[test]
    fn test_handle_ping() {
        let mut handler = RedisActionHandler::new(RedisDatabase::new());
        let resp = handler.handle(RespType::SimpleString("PING".to_string()));
        assert_eq!(resp, RespType::SimpleString("PONG".to_string()));
    }

    #[test]
    fn test_handle_unknown() {
        let mut handler = RedisActionHandler::new(RedisDatabase::new());
        let resp = handler.handle(RespType::SimpleString("UNKNOWN".to_string()));
        assert_eq!(resp, RespType::Error("Unknown command".to_string()));
    }
}
