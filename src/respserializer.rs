use crate::respparser::RespType;

#[derive(Debug, PartialEq)]
pub struct RespSerializer<W: std::io::Write> {
    writer: W,
}

impl<W: std::io::Write> RespSerializer<W> {
    pub fn new(writer: W) -> RespSerializer<W> {
        RespSerializer { writer }
    }

    pub fn serialize(&mut self, resp: &RespType) -> std::io::Result<()> {
        match resp {
            RespType::SimpleString(s) => {
                writeln!(self.writer, "+{}\r", s)
            }
            RespType::Error(s) => {
                writeln!(self.writer, "-{}\r", s)
            }
            RespType::Integer(i) => {
                writeln!(self.writer, ":{}\r", i)
            }
            RespType::BulkString(bytes) => {
                writeln!(self.writer, "${}\r", bytes.len())?;
                self.writer.write_all(bytes)?;
                writeln!(self.writer, "\r")
            }
            RespType::Array(arr) => {
                writeln!(self.writer, "*{}\r", arr.len())?;
                for resp in arr {
                    self.serialize(resp)?;
                }
                Ok(())
            }
            RespType::Null => {
                writeln!(self.writer, "$-1\r")
            }
        }
    }
}

#[cfg(test)]
mod resp_serializer_tests {
    use super::*;

    #[test]
    fn test_serialize_simple_string() {
        let mut writer = Vec::new();
        let mut serializer = RespSerializer::new(&mut writer);
        let resp = RespType::SimpleString("OK".to_string());
        serializer.serialize(&resp).unwrap();
        assert_eq!(writer, b"+OK\r\n");
    }

    #[test]
    fn test_serialize_error() {
        let mut writer = Vec::new();
        let mut serializer = RespSerializer::new(&mut writer);
        let resp = RespType::Error("Error message".to_string());
        serializer.serialize(&resp).unwrap();
        assert_eq!(writer, b"-Error message\r\n");
    }

    #[test]
    fn test_serialize_integer() {
        let mut writer = Vec::new();
        let mut serializer = RespSerializer::new(&mut writer);
        let resp = RespType::Integer(42);
        serializer.serialize(&resp).unwrap();
        assert_eq!(writer, b":42\r\n");
    }

    #[test]
    fn test_serialize_bulk_string() {
        let mut writer = Vec::new();
        let mut serializer = RespSerializer::new(&mut writer);
        let resp = RespType::BulkString(b"Hello, world!".to_vec());
        serializer.serialize(&resp).unwrap();
        assert_eq!(writer, b"$13\r\nHello, world!\r\n");
    }

    #[test]
    fn test_serialize_array() {
        let mut writer = Vec::new();
        let mut serializer = RespSerializer::new(&mut writer);
        let resp = RespType::Array(vec![
            RespType::SimpleString("OK".to_string()),
            RespType::Integer(42),
            RespType::BulkString(b"Hello, world!".to_vec()),
        ]);
        serializer.serialize(&resp).unwrap();
        assert_eq!(writer, b"*3\r\n+OK\r\n:42\r\n$13\r\nHello, world!\r\n");
    }

    #[test]
    fn test_serialize_null() {
        let mut writer = Vec::new();
        let mut serializer = RespSerializer::new(&mut writer);
        let resp = RespType::Null;
        serializer.serialize(&resp).unwrap();
        assert_eq!(writer, b"$-1\r\n");
    }
}
