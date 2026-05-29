use error_handling_solutions::{ConfigError, parse_setting, sum_fields};

// Warm-up: sum_fields (Result + ?)

#[test]
fn warmup_sum_ok() {
    assert_eq!(sum_fields("2", "3"), Ok(5));
}

#[test]
fn warmup_sum_first_bad() {
    assert!(sum_fields("x", "3").is_err());
}

#[test]
fn warmup_sum_second_bad() {
    assert!(sum_fields("2", "y").is_err());
}

#[test]
fn warmup_sum_negative() {
    assert_eq!(sum_fields("-4", "10"), Ok(6));
}

// Main: parse_setting (thiserror ConfigError)

#[test]
fn main_setting_ok() {
    assert_eq!(parse_setting("port=8080"), Ok(("port".to_string(), 8080)));
}

#[test]
fn main_setting_missing_equals() {
    assert_eq!(parse_setting("noequals"), Err(ConfigError::MissingEquals));
}

#[test]
fn main_setting_empty_key() {
    assert_eq!(parse_setting("=5"), Err(ConfigError::EmptyKey));
}

#[test]
fn main_setting_bad_value() {
    assert!(matches!(
        parse_setting("port=abc"),
        Err(ConfigError::BadValue(_))
    ));
}
