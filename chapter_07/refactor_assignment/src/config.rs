use crate::error::ParseError;

pub struct Config {
    entries: Vec<(String, Value)>,
}

impl Config {
    fn new() -> Self {
        Config {
            entries: Vec::new(),
        }
    }

    fn set(&mut self, key: String, value: Value) {
        self.entries.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

#[derive(Debug)]
pub enum Value {
    Text(String),
    Number(i32),
    Boolean(bool),
}

fn parse_line(line: &str) -> Result<(String, Value), ParseError> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(ParseError::MissingEquals);
    }

    let key = parts[0].trim().to_string();
    let value_str = parts[1].trim();

    let value = if value_str.starts_with('"') && value_str.ends_with('"') {
        Value::Text(value_str[1..value_str.len() - 1].to_string())
    } else if let Ok(n) = value_str.parse::<i32>() {
        Value::Number(n)
    } else if value_str == "true" || value_str == "false" {
        Value::Boolean(value_str == "true")
    } else {
        return Err(ParseError::InvalidValue);
    };

    Ok((key, value))
}

pub fn load_config(input: &str) -> Result<Config, ParseError> {
    let mut config = Config::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = parse_line(line)?;
        config.set(key, value);
    }
    Ok(config)
}
