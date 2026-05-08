/// Convert char to u32 (unicode code point).
pub fn char_to_u32(c: char) -> u32 {
    c as u32
}

/// Convert u32 to char (unicode code point).
pub fn u32_to_char(n: u32) -> Option<char> {
    char::from_u32(n)
}

/// Convert char to ascii u8.
pub fn char_to_ascii(c: char) -> Option<u8> {
    if c.is_ascii() {
        Some(c as u8)
    } else {
        None
    }
}

/// Convert u8 to char.
pub fn u8_to_char(n: u8) -> char {
    n as char
}

/// Convert digit char to u8.
pub fn digit_to_u8(c: char) -> Option<u8> {
    c.to_digit(10).map(|d| d as u8)
}

/// Convert u8 to digit char.
pub fn u8_to_digit(n: u8) -> Option<char> {
    if n <= 9 {
        Some((b'0' + n) as char)
    } else {
        None
    }
}

/// Convert hex char to u8.
pub fn hex_char_to_u8(c: char) -> Option<u8> {
    c.to_digit(16).map(|d| d as u8)
}

/// Convert u8 to hex char (lowercase).
pub fn u8_to_hex_char(n: u8) -> Option<char> {
    if n < 16 {
        Some(if n < 10 {
            (b'0' + n) as char
        } else {
            (b'a' + n - 10) as char
        })
    } else {
        None
    }
}

/// Convert char to uppercase.
pub fn char_to_upper(c: char) -> char {
    c.to_ascii_uppercase()
}

/// Convert char to lowercase.
pub fn char_to_lower(c: char) -> char {
    c.to_ascii_lowercase()
}
