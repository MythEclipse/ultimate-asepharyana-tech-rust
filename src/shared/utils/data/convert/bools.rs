/// Convert string to bool (flexible parsing).
pub fn to_bool(s: &str) -> bool {
    matches!(
        s.to_lowercase().trim(),
        "true" | "1" | "yes" | "on" | "enabled" | "t" | "y" | "ok" | "active"
    )
}

/// Convert to bool with Option for invalid input.
pub fn try_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().trim() {
        "true" | "1" | "yes" | "on" | "enabled" | "t" | "y" | "ok" | "active" => Some(true),
        "false" | "0" | "no" | "off" | "disabled" | "f" | "n" | "inactive" => Some(false),
        _ => None,
    }
}

/// Convert bool to string.
pub fn bool_to_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

/// Convert bool to yes/no.
pub fn bool_to_yes_no(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "no"
    }
}

/// Convert bool to on/off.
pub fn bool_to_on_off(b: bool) -> &'static str {
    if b {
        "on"
    } else {
        "off"
    }
}

/// Convert bool to enabled/disabled.
pub fn bool_to_enabled(b: bool) -> &'static str {
    if b {
        "enabled"
    } else {
        "disabled"
    }
}

/// Convert bool to active/inactive.
pub fn bool_to_active(b: bool) -> &'static str {
    if b {
        "active"
    } else {
        "inactive"
    }
}

/// Convert bool to 0/1 i32.
pub fn bool_to_int(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}

/// Convert bool to 0/1 i64.
pub fn bool_to_i64(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

/// Convert i32 to bool (0 = false, other = true).
pub fn int_to_bool(n: i32) -> bool {
    n != 0
}
