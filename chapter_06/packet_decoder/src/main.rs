const LOGIN: u8 = 0x01;
const ALERT: u8 = 0x02;
const PING: u8 = 0x03;

fn main() {
    let username = "Thomas".as_bytes();
    let mut message: Vec<u8> = vec![LOGIN];
    message.push(username.len() as u8);
    message.extend_from_slice(username);

    match parse(&message) {
        Ok(message) => print_message(message),
        Err(_) => println!("Boom"),
    }

    message[1] += 1;
    match parse(&message) {
        Ok(message) => print_message(message),
        Err(err) => handle_err(err),
    }

    let mut message = vec![ALERT];
    message.push(4);
    message.push(4);
    match parse(&message) {
        Ok(message) => print_message(message),
        Err(err) => handle_err(err),
    }

    let mut message = vec![ALERT];
    message.push(6);
    message.push(6);
    match parse(&message) {
        Ok(message) => print_message(message),
        Err(err) => handle_err(err),
    }

    let message = vec![PING];
    match parse(&message) {
        Ok(message) => print_message(message),
        Err(err) => handle_err(err),
    }

    let message = vec![0x04];
    match parse(&message) {
        Ok(message) => print_message(message),
        Err(err) => handle_err(err),
    }
}

enum Message {
    Login(String),
    Alert { code: u8, severity: u8 },
    Ping,
}

enum ParseError {
    EndOfStream,
    InvalidUtf8,
    UnknownCommand(u8),
}

fn handle_err(err: ParseError) {
    match err {
        ParseError::EndOfStream => println!("Reached end of stream before end of message"),
        ParseError::InvalidUtf8 => println!("Message contains invalid UTF8 characters"),
        ParseError::UnknownCommand(cmd) => println!("Unknown command {cmd}"),
    }
}

fn print_message(message: Message) {
    match message {
        Message::Login(username) => {
            println!("Logging in user {username}");
        }
        Message::Alert { code, severity } => {
            if severity > 5 {
                println!("Fatal Error: {code}");
            } else {
                println!("Warning: {code}");
            }
        }
        Message::Ping => println!("PingPingPing"),
    }
}

fn parse(buffer: &[u8]) -> Result<Message, ParseError> {
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
