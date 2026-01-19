pub enum ParseError {
    MissingEquals,
    InvalidValue,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::MissingEquals => write!(f, "Line missing '=' separator"),
            ParseError::InvalidValue => write!(f, "Could not parse value"),
        }
    }
}