pub enum ParseError {
    EndOfStream,
    InvalidUtf8,
    UnknownCommand(u8),
}
