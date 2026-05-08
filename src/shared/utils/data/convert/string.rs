use std::str::FromStr;
use std::fmt::Display;

/// Parse with default value.
pub fn parse_or<T: FromStr>(s: &str, default: T) -> T {
    s.parse().unwrap_or(default)
}

/// Try parse with error context.
pub fn try_parse<T: FromStr>(s: &str, name: &str) -> Result<T, String>
where
    T::Err: Display,
{
    s.parse()
        .map_err(|e| format!("Failed to parse {}: {}", name, e))
}

/// Parse i8 with default.
pub fn parse_i8(s: &str, default: i8) -> i8 {
    s.parse().unwrap_or(default)
}

/// Parse i16 with default.
pub fn parse_i16(s: &str, default: i16) -> i16 {
    s.parse().unwrap_or(default)
}

/// Parse i32 with default.
pub fn parse_i32(s: &str, default: i32) -> i32 {
    s.parse().unwrap_or(default)
}

/// Parse i64 with default.
pub fn parse_i64(s: &str, default: i64) -> i64 {
    s.parse().unwrap_or(default)
}

/// Parse i128 with default.
pub fn parse_i128(s: &str, default: i128) -> i128 {
    s.parse().unwrap_or(default)
}

/// Parse u8 with default.
pub fn parse_u8(s: &str, default: u8) -> u8 {
    s.parse().unwrap_or(default)
}

/// Parse u16 with default.
pub fn parse_u16(s: &str, default: u16) -> u16 {
    s.parse().unwrap_or(default)
}

/// Parse u32 with default.
pub fn parse_u32(s: &str, default: u32) -> u32 {
    s.parse().unwrap_or(default)
}

/// Parse u64 with default.
pub fn parse_u64(s: &str, default: u64) -> u64 {
    s.parse().unwrap_or(default)
}

/// Parse u128 with default.
pub fn parse_u128(s: &str, default: u128) -> u128 {
    s.parse().unwrap_or(default)
}

/// Parse f32 with default.
pub fn parse_f32(s: &str, default: f32) -> f32 {
    s.parse().unwrap_or(default)
}

/// Parse f64 with default.
pub fn parse_f64(s: &str, default: f64) -> f64 {
    s.parse().unwrap_or(default)
}

/// Parse usize with default.
pub fn parse_usize(s: &str, default: usize) -> usize {
    s.parse().unwrap_or(default)
}

/// Parse isize with default.
pub fn parse_isize(s: &str, default: isize) -> isize {
    s.parse().unwrap_or(default)
}

/// Convert string to Option (None for empty).
pub fn empty_to_none(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

/// Convert None to empty string.
pub fn none_to_empty(opt: Option<String>) -> String {
    opt.unwrap_or_default()
}

/// Convert Option<&str> to Option<String>.
pub fn str_to_string(opt: Option<&str>) -> Option<String> {
    opt.map(|s| s.to_string())
}

/// Convert String to Option (None if empty).
pub fn string_to_option(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Trim and convert to Option (None if whitespace only).
pub fn trim_to_option(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Convert &str to String.
pub fn str_to_owned(s: &str) -> String {
    s.to_string()
}

/// Convert String to &str (returns empty if none).
pub fn string_to_str(s: &Option<String>) -> &str {
    s.as_deref().unwrap_or("")
}
