pub mod bools;
pub mod bytes;
pub mod char;
pub mod color;
pub mod collections;
pub mod network;
pub mod numeric;
pub mod path;
pub mod pointers;
pub mod result;
pub mod string;
pub mod time;

pub use bools::*;
pub use bytes::*;
pub use char::*;
pub use color::*;
pub use collections::*;
pub use network::*;
pub use numeric::*;
pub use path::*;
pub use pointers::*;
pub use result::*;
pub use string::*;
pub use time::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_conversions() {
        assert_eq!(i64_to_u8(300), 255);
        assert_eq!(i64_to_u8(-5), 0);
        assert_eq!(i128_to_i64(i128::MAX), i64::MAX);
    }

    #[test]
    fn test_time_conversions() {
        assert_eq!(seconds_to_human(90), "1m 30s");
        assert_eq!(seconds_to_compact(86400), "1d");
    }

    #[test]
    fn test_color_conversions() {
        assert_eq!(hex_to_rgb("#ff8000"), Some((255, 128, 0)));
        assert_eq!(rgb_to_hex(255, 128, 0), "#ff8000");
    }

    #[test]
    fn test_network_conversions() {
        assert_eq!(ipv4_to_u32("192.168.1.1"), Some(0xC0A80101));
        assert_eq!(u32_to_ipv4(0xC0A80101), "192.168.1.1");
    }

    #[test]
    fn test_path_conversions() {
        let path = str_to_path("/test/path.txt");
        assert_eq!(path_extension(&path), Some("txt".to_string()));
        assert_eq!(path_filename(&path), Some("path.txt".to_string()));
    }
}
