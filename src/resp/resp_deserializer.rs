use std::io::BufRead;

use super::RespType;

pub struct RespDeserializer<R: BufRead> {
    reader: R,
}

impl<R: BufRead> RespDeserializer<R> {
    pub fn new(reader: R) -> RespDeserializer<R> {
        RespDeserializer { reader }
    }

    pub fn deserialize(&mut self) -> Result<RespType, Box<dyn std::error::Error>> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;

        match line.chars().next() {
            Some('+') => Ok(RespType::SimpleString(line[1..].trim().to_string())),
            Some('-') => Ok(RespType::Error(line[1..].trim().to_string())),
            Some(':') => Ok(RespType::Integer(line[1..].trim().parse()?)),
            Some('$') => {
                let len: isize = line[1..].trim().parse()?;
                if len == -1 {
                    Ok(RespType::Null)
                } else {
                    let mut buf = vec![0; len as usize];
                    self.reader.read_exact(&mut buf)?;

                    // Consume the next two bytes, expecting them to be '\r\n'
                    let mut crlf = [0; 2];
                    self.reader.read_exact(&mut crlf)?;
                    if &crlf != b"\r\n" {
                        return Err("Expected \\r\\n".into());
                    }

                    Ok(RespType::BulkString(buf))
                }
            }
            Some('*') => {
                let len: isize = line[1..].trim().parse()?;
                if len == -1 {
                    return Ok(RespType::Null);
                }
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(self.deserialize()?);
                }
                Ok(RespType::Array(arr))
            }
            c => Err(format!("Invalid RESP type: {:?}", c).into()),
        }
    }
}

#[cfg(test)]
mod resp_parser_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_simple_string() {
        let data = "+OK\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::SimpleString(s) => assert_eq!(s, "OK"),
            _ => panic!("Unexpected RESP type"),
        }
    }

    #[test]
    fn test_simple_string_multiple_words() {
        let data = "+Hello World\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::SimpleString(s) => assert_eq!(s, "Hello World"),
            x => panic!("Unexpected RESP type: {:?}", x),
        }
    }

    #[test]
    fn test_parse_error_message() {
        let data = "-Error message\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::Error(s) => assert_eq!(s, "Error message"),
            x => panic!("Unexpected RESP type: {:?}", x),
        }
    }

    #[test]
    fn test_parse_array_with_one_element() {
        let data = "*1\r\n$4\r\nping\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::Array(s) => {
                assert_eq!(s.len(), 1);
                assert_eq!(s[0], RespType::BulkString(b"ping".to_vec()));
            }
            x => panic!("Unexpected RESP type: {:?}", x),
        }
    }

    #[test]
    fn test_parse_array_with_multiple_elements() {
        let data = "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::Array(s) => {
                assert_eq!(s.len(), 2);
                assert_eq!(s[0], RespType::BulkString(b"echo".to_vec()));
                assert_eq!(s[1], RespType::BulkString(b"hello world".to_vec()));
            }
            x => panic!("Unexpected RESP type: {:?}", x),
        }
    }

    #[test]
    fn test_parse_array_with_multiple_elements_invalid() {
        let data = "*2\r\n$4\r\necho\r\n$5\r\nhello world\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap_err() {
            s => assert_eq!(s.to_string(), "Expected \\r\\n"),
        }
    }

    #[test]
    fn test_parse_array_with_invalid_extra_backslash() {
        let data = "*2\r\n$3\r\nget\r\n$3\\r\nkey\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap_err() {
            s => assert_eq!(s.to_string(), "invalid digit found in string"),
        }
    }

    #[test]
    fn test_parse_empty_bulk_string() {
        let data = "$0\r\n\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::BulkString(s) => assert_eq!(s, b""),
            x => panic!("Unexpected RESP type: {:?}", x),
        }
    }

    #[test]
    fn test_parse_null_bulk_string() {
        let data = "$-1\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::Null => assert!(true),
            x => panic!("Unexpected RESP type: {:?}", x),
        }
    }

    #[test]
    fn test_parse_null() {
        let data = "*-1\r\n";
        let mut parser = RespDeserializer::new(Cursor::new(data));
        match parser.deserialize().unwrap() {
            RespType::Null => assert!(true),
            x => panic!("Unexpected RESP type: {:?}", x),
        }
    }
}
