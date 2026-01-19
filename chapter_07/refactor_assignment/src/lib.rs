mod config;
mod error;

pub use config::{Config, Value, load_config};
pub use error::ParseError;
