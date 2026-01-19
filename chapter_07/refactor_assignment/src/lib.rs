mod config;
mod error;

pub use config::{Config, load_config, Value};
pub use error::ParseError;