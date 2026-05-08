use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Convert seconds to human readable duration.
pub fn seconds_to_human(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs < 86400 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else if secs < 604800 {
        format!("{}d {}h", secs / 86400, (secs % 86400) / 3600)
    } else if secs < 2592000 {
        format!("{}w {}d", secs / 604800, (secs % 604800) / 86400)
    } else if secs < 31536000 {
        format!("{}mo {}d", secs / 2592000, (secs % 2592000) / 86400)
    } else {
        format!("{}y {}mo", secs / 31536000, (secs % 31536000) / 2592000)
    }
}

/// Convert seconds to compact human readable.
pub fn seconds_to_compact(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86400 {
        format!("{}h", secs / 3600)
    } else if secs < 604800 {
        format!("{}d", secs / 86400)
    } else if secs < 2592000 {
        format!("{}w", secs / 604800)
    } else if secs < 31536000 {
        format!("{}mo", secs / 2592000)
    } else {
        format!("{}y", secs / 31536000)
    }
}

/// Convert milliseconds to human readable.
pub fn ms_to_human(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        seconds_to_human(ms / 1000)
    }
}

/// Convert microseconds to human readable.
pub fn us_to_human(us: u64) -> String {
    if us < 1000 {
        format!("{}μs", us)
    } else if us < 1_000_000 {
        format!("{:.2}ms", us as f64 / 1000.0)
    } else {
        seconds_to_human(us / 1_000_000)
    }
}

/// Convert nanoseconds to human readable.
pub fn ns_to_human(ns: u64) -> String {
    if ns < 1_000 {
        format!("{}ns", ns)
    } else if ns < 1_000_000 {
        format!("{:.2}μs", ns as f64 / 1_000.0)
    } else if ns < 1_000_000_000 {
        format!("{:.2}ms", ns as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", ns as f64 / 1_000_000_000.0)
    }
}

/// Convert seconds to Duration.
pub fn secs_to_duration(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

/// Convert milliseconds to Duration.
pub fn ms_to_duration(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

/// Convert microseconds to Duration.
pub fn us_to_duration(us: u64) -> Duration {
    Duration::from_micros(us)
}

/// Convert nanoseconds to Duration.
pub fn ns_to_duration(ns: u64) -> Duration {
    Duration::from_nanos(ns)
}

/// Convert Duration to seconds.
pub fn duration_to_secs(d: Duration) -> u64 {
    d.as_secs()
}

/// Convert Duration to milliseconds.
pub fn duration_to_ms(d: Duration) -> u128 {
    d.as_millis()
}

/// Convert Duration to microseconds.
pub fn duration_to_us(d: Duration) -> u128 {
    d.as_micros()
}

/// Convert Duration to nanoseconds.
pub fn duration_to_ns(d: Duration) -> u128 {
    d.as_nanos()
}

/// Convert Duration to f64 seconds.
pub fn duration_to_secs_f64(d: Duration) -> f64 {
    d.as_secs_f64()
}

/// Convert f64 seconds to Duration.
pub fn secs_f64_to_duration(secs: f64) -> Duration {
    Duration::from_secs_f64(secs.max(0.0))
}

/// Get SystemTime as Unix timestamp (seconds).
pub fn system_time_to_unix(t: SystemTime) -> u64 {
    t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

/// Get SystemTime as Unix timestamp (milliseconds).
pub fn system_time_to_unix_ms(t: SystemTime) -> u128 {
    t.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()
}

/// Convert Unix timestamp to SystemTime.
pub fn unix_to_system_time(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}

/// Convert Unix milliseconds to SystemTime.
pub fn unix_ms_to_system_time(ms: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(ms)
}
