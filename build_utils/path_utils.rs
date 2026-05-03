use crate::build_utils::constants::get_dynamic_regex;

pub fn generate_default_description(path_str: &str, method: &str) -> String {
    let path_segments: Vec<&str> = path_str.trim_matches('/').split('/').collect();
    let last_segment = path_segments.last().unwrap_or(&"");
    let second_last_segment = if path_segments.len() > 1 {
        path_segments[path_segments.len() - 2]
    } else {
        ""
    };

    match *last_segment {
        "search" => format!(
            "Searches for {} based on query parameters.",
            second_last_segment
        ),
        "detail" => format!(
            "Retrieves details for a specific {} by ID.",
            second_last_segment
        ),
        "list" => format!("Retrieves a list of {}.", second_last_segment),
        "index" => {
            if second_last_segment.is_empty() {
                format!(
                    "Handles {} requests for the root API endpoint.",
                    method.to_uppercase()
                )
            } else {
                format!(
                    "Handles {} requests for the {} index endpoint.",
                    method.to_uppercase(),
                    second_last_segment
                )
            }
        }
        "[slug]" => format!(
            "Retrieves details for a specific {} by slug.",
            second_last_segment
        ),
        "[[...file]]" => format!(
            "Handles file operations (upload/download) for {}.",
            second_last_segment
        ),
        _ => format!(
            "Handles {} requests for the {} endpoint.",
            method.to_uppercase(),
            path_str
        ),
    }
}

pub fn sanitize_operation_id(path_str: &str) -> String {
    let s = path_str.replace([std::path::MAIN_SEPARATOR, '-'], "_");
    let s = get_dynamic_regex()
        .replace_all(&s, |caps: &regex::Captures| {
            let inner = &caps[1];
            if inner.starts_with("...") {
                "_catch_all".to_string()
            } else {
                format!("_{}", inner)
            }
        })
        .to_string();
    s.trim_matches('_').replace("__", "_")
}

pub fn sanitize_tag(path_str: &str) -> String {
    let first_part = path_str
        .split(std::path::MAIN_SEPARATOR)
        .next()
        .unwrap_or("");
    let s = first_part.replace('-', "_");
    let s = get_dynamic_regex()
        .replace_all(&s, |caps: &regex::Captures| {
            let inner = &caps[1];
            if inner.starts_with("...") {
                "_catch_all".to_string()
            } else {
                format!("_{}", inner)
            }
        })
        .to_string();
    s.trim_matches('_').to_string()
}

pub fn is_dynamic_route_content(content: &str) -> bool {
    content.contains("//! DYNAMIC_ROUTE")
}

pub fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

pub fn sanitize_module_name(name: &str) -> String {
    let sanitized = name
        .trim_matches(|c| c == '[' || c == ']')
        .replace('-', "_");
    if is_rust_keyword(&sanitized) {
        format!("r#{}", sanitized)
    } else {
        sanitized
    }
}

pub fn compute_module_path_prefix(current_dir: &std::path::Path, root_api_path: &std::path::Path) -> Result<String, anyhow::Error> {
    let relative_path = current_dir
        .strip_prefix(root_api_path)
        .unwrap_or(std::path::Path::new(""));
    let relative_path_str = relative_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path encoding for directory: {:?}", current_dir))?
        .replace(std::path::MAIN_SEPARATOR, "::")
        .replace('-', "_");

    let module_path_prefix = if relative_path_str.is_empty() {
        "crate::routes::api".to_string()
    } else {
        let sanitized_segments: Vec<String> = relative_path_str
            .split("::")
            .map(|s| sanitize_module_name(&s.replace("[", "").replace("]", "")))
            .collect();
        format!("crate::routes::api::{}", sanitized_segments.join("::"))
    };

    Ok(module_path_prefix)
}
