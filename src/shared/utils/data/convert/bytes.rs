/// Convert bytes to hex string (lowercase).
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Convert bytes to hex string (uppercase).
pub fn bytes_to_hex_upper(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

/// Convert hex string to bytes.
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

/// Convert bytes to base64.
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
}

/// Convert base64 to bytes.
pub fn base64_to_bytes(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s)
}

/// Convert bytes to binary string.
pub fn bytes_to_binary(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert bytes to octal string.
pub fn bytes_to_octal(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:03o}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert u8 to binary string.
pub fn u8_to_binary(n: u8) -> String {
    format!("{:08b}", n)
}

/// Convert u16 to binary string.
pub fn u16_to_binary(n: u16) -> String {
    format!("{:016b}", n)
}

/// Convert u32 to binary string.
pub fn u32_to_binary(n: u32) -> String {
    format!("{:032b}", n)
}

/// Convert u64 to binary string.
pub fn u64_to_binary(n: u64) -> String {
    format!("{:064b}", n)
}

/// Convert binary string to u64.
pub fn binary_to_u64(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(&s.replace(" ", ""), 2)
}

/// Swap endianness of u16.
pub fn swap_endian_u16(n: u16) -> u16 {
    n.swap_bytes()
}

/// Swap endianness of u32.
pub fn swap_endian_u32(n: u32) -> u32 {
    n.swap_bytes()
}

/// Swap endianness of u64.
pub fn swap_endian_u64(n: u64) -> u64 {
    n.swap_bytes()
}

/// Swap endianness of u128.
pub fn swap_endian_u128(n: u128) -> u128 {
    n.swap_bytes()
}

/// Convert to big endian bytes.
pub fn u32_to_be_bytes(n: u32) -> [u8; 4] {
    n.to_be_bytes()
}

/// Convert to little endian bytes.
pub fn u32_to_le_bytes(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

/// Convert from big endian bytes.
pub fn be_bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

/// Convert from little endian bytes.
pub fn le_bytes_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
}

/// Convert to big endian bytes.
pub fn u64_to_be_bytes(n: u64) -> [u8; 8] {
    n.to_be_bytes()
}

/// Convert to little endian bytes.
pub fn u64_to_le_bytes(n: u64) -> [u8; 8] {
    n.to_le_bytes()
}

/// Convert from big endian bytes.
pub fn be_bytes_to_u64(bytes: [u8; 8]) -> u64 {
    u64::from_be_bytes(bytes)
}

/// Convert from little endian bytes.
pub fn le_bytes_to_u64(bytes: [u8; 8]) -> u64 {
    u64::from_le_bytes(bytes)
}
