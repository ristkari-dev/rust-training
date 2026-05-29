//! Lesson 14 — reference solutions.

use thiserror::Error;

pub fn sum_fields(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error("missing '=' in setting")]
    MissingEquals,
    #[error("key is empty")]
    EmptyKey,
    #[error("value is not a valid integer: {0}")]
    BadValue(#[from] std::num::ParseIntError),
}

pub fn parse_setting(input: &str) -> Result<(String, i32), ConfigError> {
    let (key, value) = input.split_once('=').ok_or(ConfigError::MissingEquals)?;
    if key.is_empty() {
        return Err(ConfigError::EmptyKey);
    }
    let value: i32 = value.parse()?;
    Ok((key.to_string(), value))
}
