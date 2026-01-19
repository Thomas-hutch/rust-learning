use packet_decoder_refactored::{ALERT, LOGIN, Message, PING, ParseError, parse};

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
