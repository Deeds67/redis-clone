use crate::resp_parser::RespType;

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

        println!("resp: {:?}", resp);

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
    use crate::resp_parser::RespType;
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
}
