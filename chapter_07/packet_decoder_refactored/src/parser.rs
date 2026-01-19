use crate::messages::{Message, LOGIN, ALERT, PING};
use crate::error::ParseError;

pub fn parse(buffer: &[u8]) -> Result<Message, ParseError> {
    if buffer.is_empty() {
        return Err(ParseError::EndOfStream);
    }

    match buffer[0] {
        LOGIN => {
            if buffer.len() < 2 {
                return Err(ParseError::EndOfStream);
            }

            let uname_len = buffer[1] as usize;
            let minimum_required_length = uname_len + 2;
            if buffer.len() < minimum_required_length {
                return Err(ParseError::EndOfStream);
            }

            let uname = &buffer[2..minimum_required_length];

            match String::from_utf8(uname.to_vec()) {
                Ok(s) => Ok(Message::Login(s)),
                Err(_) => Err(ParseError::InvalidUtf8),
            }
        }
        ALERT => {
            if buffer.len() < 3 {
                Err(ParseError::EndOfStream)
            } else {
                Ok(Message::Alert {
                    code: buffer[1],
                    severity: buffer[2],
                })
            }
        }
        PING => Ok(Message::Ping),
        other => Err(ParseError::UnknownCommand(other)),
    }
}