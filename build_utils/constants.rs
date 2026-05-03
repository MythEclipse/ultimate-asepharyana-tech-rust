use once_cell::sync::Lazy;
use regex::Regex;

pub static HANDLER_FN_REGEX: Lazy<Result<Regex, regex::Error>> =
    Lazy::new(|| Regex::new(r"pub async fn\s+([a-zA-Z0-9_]+)\s*\("));

pub static ENDPOINT_METADATA_REGEX: Lazy<Result<Regex, regex::Error>> = Lazy::new(|| {
    Regex::new(
      r#"const\s+(ENDPOINT_METHOD|ENDPOINT_PATH|ENDPOINT_DESCRIPTION|ENDPOINT_TAG|OPERATION_ID|SUCCESS_RESPONSE_BODY):\s*&\s*str\s*=\s*"([^"]*)";"#
    )
});

pub static DYNAMIC_REGEX: Lazy<Result<Regex, regex::Error>> =
    Lazy::new(|| Regex::new(r"\[([^\]]+)\]"));

#[inline]
pub fn get_handler_fn_regex() -> &'static Regex {
    HANDLER_FN_REGEX.as_ref().expect("Valid regex")
}

#[inline]
pub fn get_endpoint_metadata_regex() -> &'static Regex {
    ENDPOINT_METADATA_REGEX.as_ref().expect("Valid regex")
}

#[inline]
pub fn get_dynamic_regex() -> &'static Regex {
    DYNAMIC_REGEX.as_ref().expect("Valid regex")
}
