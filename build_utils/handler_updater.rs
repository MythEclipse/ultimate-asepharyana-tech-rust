use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::build_utils::constants::{ENDPOINT_METADATA_REGEX, HANDLER_FN_REGEX};
use crate::build_utils::handler_template::generate_handler_template;
use crate::build_utils::path_utils::{
    generate_default_description, sanitize_operation_id, sanitize_tag,
};

pub struct HandlerRouteInfo {
    #[allow(dead_code)]
    pub func_name: String,
    #[allow(dead_code)]
    pub handler_module_path: String,
}

pub fn update_handler_file(
    path: &Path,
    module_path_prefix: &str,
    root_api_path: &Path,
) -> Result<Vec<HandlerRouteInfo>> {
    let initial_content = read_and_check_file(path)?;

    let content = match initial_content {
        Some(c) => c,
        None => {
            handle_empty_file(path, root_api_path)?;
            return Ok(Vec::new());
        }
    };

    let file_stem = get_file_stem(path)?;
    let doc_comment = get_doc_comment(&content);

    // Simplified extraction: just find the main handler function
    let metadata_map = extract_and_normalize_metadata(&content, path, root_api_path, &file_stem, doc_comment)?;
    
    let func_name = HANDLER_FN_REGEX
        .captures(&content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| file_stem.to_string());

    let res = HandlerRouteInfo {
        func_name: func_name.clone(),
        handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
    };

    // Update register_routes if it exists or needs to be added
    let final_content = generate_and_update_register_routes(&content, &content, "get", &file_stem, &metadata_map)?;
    
    if content != final_content {
        fs::write(path, final_content)?;
    }

    Ok(vec![res])
}

fn read_and_check_file(path: &Path) -> Result<Option<String>> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))?;

    if content.contains("// This register_routes is manually maintained") {
        Ok(None)
    } else {
        Ok(Some(content))
    }
}

fn handle_empty_file(path: &Path, root_api_path: &Path) -> Result<()> {
    generate_handler_template(path, root_api_path)?;
    Ok(())
}

fn get_file_stem(path: &Path) -> Result<String> {
    Ok(path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Could not get file stem from {:?}", path))?
        .replace(['[', ']'], "")
        .replace('-', "_")
        .to_string())
}

fn get_doc_comment(content: &str) -> Option<String> {
    let register_pos = content
        .find("pub fn register_routes")
        .unwrap_or(content.len());
    let before = &content[..register_pos];
    let lines: Vec<&str> = before.lines().rev().collect();
    let mut doc_lines = Vec::new();
    for line in lines {
        if line.trim_start().starts_with("//!") {
            doc_lines.push(line.trim_start().strip_prefix("//!").unwrap_or(line).trim());
        } else if !line.trim().is_empty() {
            break;
        }
    }
    doc_lines.reverse();
    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join(" "))
    }
}

fn generate_and_update_register_routes(
    content: &str,
    original_content: &str,
    _http_method: &str,
    _file_stem: &str,
    metadata: &HashMap<String, String>,
) -> Result<String> {
    let method = metadata.get("ENDPOINT_METHOD").cloned().unwrap_or_else(|| "get".to_string());
    let path = metadata.get("ENDPOINT_PATH").cloned().unwrap_or_else(|| "/".to_string());
    
    let func_name = HANDLER_FN_REGEX
        .captures(content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| _file_stem.to_string());

    let route_call = format!(".route(\"{}\", axum::routing::{}({}))", path, method, func_name);
    let new_register_fn = format!(
        "pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {{\n    router{}\n}}",
        route_call
    );

    let mut final_content = content.to_string();

    let register_regex =
        Regex::new(r"(?s)pub fn register_routes\(.*?\)\s*->\s*Router<Arc<AppState>>\s*\{.*\}\s*")
            .unwrap();

    if let Some(existing_register) = register_regex.find(&final_content) {
        let route_count = existing_register.as_str().matches(".route(").count();
        if route_count > 1 {
            final_content = original_content.to_string();
        } else {
            final_content = register_regex.replace_all(&final_content, "").to_string();
            final_content = final_content.trim_end().to_string();
            final_content.push_str("\n\n");
            final_content.push_str(&new_register_fn);
        }
    } else {
        final_content = final_content.trim_end().to_string();
        final_content.push_str("\n\n");
        final_content.push_str(&new_register_fn);
    }
    Ok(final_content)
}

fn extract_and_normalize_metadata(
    content: &str,
    path: &Path,
    root_api_path: &Path,
    file_stem: &str,
    doc_comment: Option<String>,
) -> Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();

    for cap in ENDPOINT_METADATA_REGEX.captures_iter(content) {
        metadata.insert(cap[1].to_string(), cap[2].to_string());
    }

    let relative_path_no_ext = path.strip_prefix(root_api_path).unwrap().with_extension("");
    let relative_path_str = relative_path_no_ext.to_str().unwrap();

    let default_tag = sanitize_tag(relative_path_str);
    metadata.entry("ENDPOINT_TAG".to_string()).or_insert(if default_tag.is_empty() { "api".to_string() } else { default_tag });

    let operation_id = sanitize_operation_id(relative_path_str);
    metadata.entry("OPERATION_ID".to_string()).or_insert(operation_id);
    metadata.entry("ENDPOINT_METHOD".to_string()).or_insert("get".to_string());

    let default_route_path = relative_path_no_ext.to_str().unwrap().replace("\\", "/");
    let mut route_path = metadata.entry("ENDPOINT_PATH".to_string()).or_insert(default_route_path).clone();

    if file_stem == "index" && route_path.ends_with("/index") {
        route_path = route_path.strip_suffix("/index").unwrap_or("/").to_string();
    }
    
    // Simplistic normalization
    if !route_path.starts_with("/api/") {
        route_path = format!("/api/{}", route_path.trim_start_matches('/'));
    }
    metadata.insert("ENDPOINT_PATH".to_string(), route_path.clone());

    let http_method = metadata.get("ENDPOINT_METHOD").unwrap().clone();
    let default_description = doc_comment.unwrap_or_else(|| generate_default_description(&route_path, &http_method));
    metadata.entry("ENDPOINT_DESCRIPTION".to_string()).or_insert(default_description);

    Ok(metadata)
}
