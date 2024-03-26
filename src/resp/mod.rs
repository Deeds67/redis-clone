mod resp_deserializer;
mod resp_serializer;

pub use resp_deserializer::RespDeserializer;
pub use resp_serializer::RespSerializer;

#[derive(Debug, PartialEq)]
pub enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespType>),
    Null,
}