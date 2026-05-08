/// Safe i8 to u8.
pub fn i8_to_u8(n: i8) -> u8 {
    n.max(0) as u8
}

/// Safe i8 to i16.
pub fn i8_to_i16(n: i8) -> i16 {
    n as i16
}

/// Safe i8 to i32.
pub fn i8_to_i32(n: i8) -> i32 {
    n as i32
}

/// Safe i8 to i64.
pub fn i8_to_i64(n: i8) -> i64 {
    n as i64
}

/// Safe i8 to i128.
pub fn i8_to_i128(n: i8) -> i128 {
    n as i128
}

/// Safe i8 to f32.
pub fn i8_to_f32(n: i8) -> f32 {
    n as f32
}

/// Safe i8 to f64.
pub fn i8_to_f64(n: i8) -> f64 {
    n as f64
}

/// Safe i16 to i8 (saturates).
pub fn i16_to_i8(n: i16) -> i8 {
    n.clamp(i8::MIN as i16, i8::MAX as i16) as i8
}

/// Safe i16 to u8 (saturates).
pub fn i16_to_u8(n: i16) -> u8 {
    n.clamp(0, u8::MAX as i16) as u8
}

/// Safe i16 to u16.
pub fn i16_to_u16(n: i16) -> u16 {
    n.max(0) as u16
}

/// Safe i16 to i32.
pub fn i16_to_i32(n: i16) -> i32 {
    n as i32
}

/// Safe i16 to i64.
pub fn i16_to_i64(n: i16) -> i64 {
    n as i64
}

/// Safe i16 to i128.
pub fn i16_to_i128(n: i16) -> i128 {
    n as i128
}

/// Safe i32 to i8 (saturates).
pub fn i32_to_i8(n: i32) -> i8 {
    n.clamp(i8::MIN as i32, i8::MAX as i32) as i8
}

/// Safe i32 to i16 (saturates).
pub fn i32_to_i16(n: i32) -> i16 {
    n.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// Safe i32 to u8 (saturates).
pub fn i32_to_u8(n: i32) -> u8 {
    n.clamp(0, u8::MAX as i32) as u8
}

/// Safe i32 to u16 (saturates).
pub fn i32_to_u16(n: i32) -> u16 {
    n.clamp(0, u16::MAX as i32) as u16
}

/// Safe i32 to u32.
pub fn i32_to_u32(n: i32) -> u32 {
    n.max(0) as u32
}

/// Safe i32 to i64.
pub fn i32_to_i64(n: i32) -> i64 {
    n as i64
}

/// Safe i32 to i128.
pub fn i32_to_i128(n: i32) -> i128 {
    n as i128
}

/// Safe i32 to usize.
pub fn i32_to_usize(n: i32) -> usize {
    n.max(0) as usize
}

/// Safe i32 to f32.
pub fn i32_to_f32(n: i32) -> f32 {
    n as f32
}

/// Safe i32 to f64.
pub fn i32_to_f64(n: i32) -> f64 {
    n as f64
}

/// Safe i64 to i8 (saturates).
pub fn i64_to_i8(n: i64) -> i8 {
    n.clamp(i8::MIN as i64, i8::MAX as i64) as i8
}

/// Safe i64 to i16 (saturates).
pub fn i64_to_i16(n: i64) -> i16 {
    n.clamp(i16::MIN as i64, i16::MAX as i64) as i16
}

/// Safe i64 to i32 (saturates).
pub fn i64_to_i32(n: i64) -> i32 {
    n.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

/// Safe i64 to u8 (saturates).
pub fn i64_to_u8(n: i64) -> u8 {
    n.clamp(0, u8::MAX as i64) as u8
}

/// Safe i64 to u16 (saturates).
pub fn i64_to_u16(n: i64) -> u16 {
    n.clamp(0, u16::MAX as i64) as u16
}

/// Safe i64 to u32 (saturates).
pub fn i64_to_u32(n: i64) -> u32 {
    n.clamp(0, u32::MAX as i64) as u32
}

/// Safe i64 to u64.
pub fn i64_to_u64(n: i64) -> u64 {
    n.max(0) as u64
}

/// Safe i64 to usize.
pub fn i64_to_usize(n: i64) -> usize {
    n.max(0) as usize
}

/// Safe i64 to i128.
pub fn i64_to_i128(n: i64) -> i128 {
    n as i128
}

/// Safe i64 to u128.
pub fn i64_to_u128(n: i64) -> u128 {
    n.max(0) as u128
}

/// Safe i64 to f32.
pub fn i64_to_f32(n: i64) -> f32 {
    n as f32
}

/// Safe i64 to f64.
pub fn i64_to_f64(n: i64) -> f64 {
    n as f64
}

/// Safe i128 to i8 (saturates).
pub fn i128_to_i8(n: i128) -> i8 {
    n.clamp(i8::MIN as i128, i8::MAX as i128) as i8
}

/// Safe i128 to i16 (saturates).
pub fn i128_to_i16(n: i128) -> i16 {
    n.clamp(i16::MIN as i128, i16::MAX as i128) as i16
}

/// Safe i128 to i32 (saturates).
pub fn i128_to_i32(n: i128) -> i32 {
    n.clamp(i32::MIN as i128, i32::MAX as i128) as i32
}

/// Safe i128 to i64 (saturates).
pub fn i128_to_i64(n: i128) -> i64 {
    n.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

/// Safe i128 to u128.
pub fn i128_to_u128(n: i128) -> u128 {
    n.max(0) as u128
}

/// Safe i128 to usize.
pub fn i128_to_usize(n: i128) -> usize {
    n.clamp(0, usize::MAX as i128) as usize
}

/// u8 to i8 (may overflow to negative).
pub fn u8_to_i8_wrap(n: u8) -> i8 {
    n as i8
}

/// u8 to i8 (saturates at i8::MAX).
pub fn u8_to_i8_sat(n: u8) -> i8 {
    n.min(i8::MAX as u8) as i8
}

/// u8 to i16.
pub fn u8_to_i16(n: u8) -> i16 {
    n as i16
}

/// u8 to i32.
pub fn u8_to_i32(n: u8) -> i32 {
    n as i32
}

/// u8 to i64.
pub fn u8_to_i64(n: u8) -> i64 {
    n as i64
}

/// u8 to u16.
pub fn u8_to_u16(n: u8) -> u16 {
    n as u16
}

/// u8 to u32.
pub fn u8_to_u32(n: u8) -> u32 {
    n as u32
}

/// u8 to u64.
pub fn u8_to_u64(n: u8) -> u64 {
    n as u64
}

/// u8 to usize.
pub fn u8_to_usize(n: u8) -> usize {
    n as usize
}

/// u8 to f32.
pub fn u8_to_f32(n: u8) -> f32 {
    n as f32
}

/// u8 to f64.
pub fn u8_to_f64(n: u8) -> f64 {
    n as f64
}

/// u16 to u8 (saturates).
pub fn u16_to_u8(n: u16) -> u8 {
    n.min(u8::MAX as u16) as u8
}

/// u16 to i16 (saturates).
pub fn u16_to_i16(n: u16) -> i16 {
    n.min(i16::MAX as u16) as i16
}

/// u16 to i32.
pub fn u16_to_i32(n: u16) -> i32 {
    n as i32
}

/// u16 to i64.
pub fn u16_to_i64(n: u16) -> i64 {
    n as i64
}

/// u16 to u32.
pub fn u16_to_u32(n: u16) -> u32 {
    n as u32
}

/// u16 to u64.
pub fn u16_to_u64(n: u16) -> u64 {
    n as u64
}

/// u16 to usize.
pub fn u16_to_usize(n: u16) -> usize {
    n as usize
}

/// u32 to u8 (saturates).
pub fn u32_to_u8(n: u32) -> u8 {
    n.min(u8::MAX as u32) as u8
}

/// u32 to u16 (saturates).
pub fn u32_to_u16(n: u32) -> u16 {
    n.min(u16::MAX as u32) as u16
}

/// u32 to i32 (saturates).
pub fn u32_to_i32(n: u32) -> i32 {
    n.min(i32::MAX as u32) as i32
}

/// u32 to i64.
pub fn u32_to_i64(n: u32) -> i64 {
    n as i64
}

/// u32 to u64.
pub fn u32_to_u64(n: u32) -> u64 {
    n as u64
}

/// u32 to usize.
pub fn u32_to_usize(n: u32) -> usize {
    n as usize
}

/// u32 to f32.
pub fn u32_to_f32(n: u32) -> f32 {
    n as f32
}

/// u32 to f64.
pub fn u32_to_f64(n: u32) -> f64 {
    n as f64
}

/// u64 to u8 (saturates).
pub fn u64_to_u8(n: u64) -> u8 {
    n.min(u8::MAX as u64) as u8
}

/// u64 to u16 (saturates).
pub fn u64_to_u16(n: u64) -> u16 {
    n.min(u16::MAX as u64) as u16
}

/// u64 to u32 (saturates).
pub fn u64_to_u32(n: u64) -> u32 {
    n.min(u32::MAX as u64) as u32
}

/// u64 to i64 (saturates).
pub fn u64_to_i64(n: u64) -> i64 {
    n.min(i64::MAX as u64) as i64
}

/// u64 to i128.
pub fn u64_to_i128(n: u64) -> i128 {
    n as i128
}

/// u64 to u128.
pub fn u64_to_u128(n: u64) -> u128 {
    n as u128
}

/// u64 to usize (may truncate on 32-bit).
pub fn u64_to_usize(n: u64) -> usize {
    n as usize
}

/// u64 to f64.
pub fn u64_to_f64(n: u64) -> f64 {
    n as f64
}

/// u128 to u64 (saturates).
pub fn u128_to_u64(n: u128) -> u64 {
    n.min(u64::MAX as u128) as u64
}

/// u128 to i128 (saturates).
pub fn u128_to_i128(n: u128) -> i128 {
    n.min(i128::MAX as u128) as i128
}

/// u128 to usize (saturates).
pub fn u128_to_usize(n: u128) -> usize {
    n.min(usize::MAX as u128) as usize
}

/// usize to i32 (saturates).
pub fn usize_to_i32(n: usize) -> i32 {
    n.min(i32::MAX as usize) as i32
}

/// usize to i64.
pub fn usize_to_i64(n: usize) -> i64 {
    n as i64
}

/// usize to u32 (saturates on 64-bit).
pub fn usize_to_u32(n: usize) -> u32 {
    n.min(u32::MAX as usize) as u32
}

/// usize to u64.
pub fn usize_to_u64(n: usize) -> u64 {
    n as u64
}

/// isize to i32 (saturates).
pub fn isize_to_i32(n: isize) -> i32 {
    n.clamp(i32::MIN as isize, i32::MAX as isize) as i32
}

/// isize to i64.
pub fn isize_to_i64(n: isize) -> i64 {
    n as i64
}

/// isize to usize.
pub fn isize_to_usize(n: isize) -> usize {
    n.max(0) as usize
}

/// f64 to f32 (may lose precision).
pub fn f64_to_f32(n: f64) -> f32 {
    n as f32
}

/// f32 to f64.
pub fn f32_to_f64(n: f32) -> f64 {
    n as f64
}

/// f64 to i64 (truncates).
pub fn f64_to_i64(n: f64) -> i64 {
    n.clamp(i64::MIN as f64, i64::MAX as f64) as i64
}

/// f64 to i32 (truncates).
pub fn f64_to_i32(n: f64) -> i32 {
    n.clamp(i32::MIN as f64, i32::MAX as f64) as i32
}

/// f64 to u64 (truncates).
pub fn f64_to_u64(n: f64) -> u64 {
    n.clamp(0.0, u64::MAX as f64) as u64
}

/// f64 to u32 (truncates).
pub fn f64_to_u32(n: f64) -> u32 {
    n.clamp(0.0, u32::MAX as f64) as u32
}

/// f32 to i32 (truncates).
pub fn f32_to_i32(n: f32) -> i32 {
    n.clamp(i32::MIN as f32, i32::MAX as f32) as i32
}

/// Round f64 to n decimal places.
pub fn round_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).round() / factor
}

/// Round f32 to n decimal places.
pub fn round_f32(n: f32, decimals: u32) -> f32 {
    let factor = 10_f32.powi(decimals as i32);
    (n * factor).round() / factor
}

/// Truncate f64 to n decimal places.
pub fn trunc_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).trunc() / factor
}

/// Ceil f64 to n decimal places.
pub fn ceil_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).ceil() / factor
}

/// Floor f64 to n decimal places.
pub fn floor_f64(n: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (n * factor).floor() / factor
}

/// Check if f64 is essentially zero.
pub fn is_zero(n: f64, epsilon: f64) -> bool {
    n.abs() < epsilon
}

/// Compare two f64 for approximate equality.
pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Check if f64 is NaN.
pub fn is_nan(n: f64) -> bool {
    n.is_nan()
}

/// Check if f64 is infinite.
pub fn is_infinite(n: f64) -> bool {
    n.is_infinite()
}

/// Check if f64 is finite.
pub fn is_finite(n: f64) -> bool {
    n.is_finite()
}

/// Convert NaN to 0.
pub fn nan_to_zero(n: f64) -> f64 {
    if n.is_nan() {
        0.0
    } else {
        n
    }
}

/// Convert NaN to default.
pub fn nan_to_default(n: f64, default: f64) -> f64 {
    if n.is_nan() {
        default
    } else {
        n
    }
}
