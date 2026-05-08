use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Convert &str to PathBuf.
pub fn str_to_path(s: &str) -> PathBuf {
    PathBuf::from(s)
}

/// Convert String to PathBuf.
pub fn string_to_path(s: String) -> PathBuf {
    PathBuf::from(s)
}

/// Convert PathBuf to String (lossy).
pub fn path_to_string(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

/// Convert PathBuf to Option<String> (None if not valid UTF-8).
pub fn path_to_string_strict(p: &Path) -> Option<String> {
    p.to_str().map(String::from)
}

/// Convert &str to &Path.
pub fn str_to_path_ref(s: &str) -> &Path {
    Path::new(s)
}

/// Convert OsStr to String (lossy).
pub fn os_str_to_string(s: &OsStr) -> String {
    s.to_string_lossy().to_string()
}

/// Convert OsString to String (lossy).
pub fn os_string_to_string(s: OsString) -> String {
    s.to_string_lossy().to_string()
}

/// Convert String to OsString.
pub fn string_to_os_string(s: String) -> OsString {
    OsString::from(s)
}

/// Convert &str to &OsStr.
pub fn str_to_os_str(s: &str) -> &OsStr {
    OsStr::new(s)
}

/// Get file extension as String.
pub fn path_extension(p: &Path) -> Option<String> {
    p.extension().and_then(|e| e.to_str()).map(String::from)
}

/// Get file name as String.
pub fn path_filename(p: &Path) -> Option<String> {
    p.file_name().and_then(|n| n.to_str()).map(String::from)
}

/// Get parent directory as PathBuf.
pub fn path_parent(p: &Path) -> Option<PathBuf> {
    p.parent().map(PathBuf::from)
}
