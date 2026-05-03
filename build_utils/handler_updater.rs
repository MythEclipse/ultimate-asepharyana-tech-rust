use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::build_utils::constants::{get_dynamic_regex, get_endpoint_metadata_regex, get_handler_fn_regex};
use crate::build_utils::handler_template::generate_handler_template;
use crate::build_utils::path_utils::{
    generate_default_description, sanitize_operation_id, sanitize_tag,
};

pub struct HandlerRouteInfo {
    #[allow(dead_code)]
    pub func_name: String,
    #[allow(dead_code)]
    pub handler_module_path: String,
    pub discovered_schemas: Vec<String>,
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
    
    let func_name = get_handler_fn_regex()
        .captures(&content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| file_stem.to_string());

    // Update register_routes if it exists or needs to be added
    let mut final_content = generate_and_update_register_routes(&content, &content, "get", &file_stem, &metadata_map)?;
    
    // Inject OpenAPI metadata
    final_content = inject_openapi_metadata(&final_content, &metadata_map, &func_name)?;

    let res = HandlerRouteInfo {
        func_name: func_name.clone(),
        handler_module_path: format!("{}::{}", module_path_prefix, file_stem),
        discovered_schemas: extract_discovered_schemas(&final_content, module_path_prefix, &file_stem),
    };

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
    
    let func_name = get_handler_fn_regex()
        .captures(content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| _file_stem.to_string());

    // Standardize path for Axum: replace [param] with {param} (Axum 0.8+)
    let axum_path = get_dynamic_regex().replace_all(&path, "{$1}").to_string();

    let route_call = format!(".route(\"{}\", axum::routing::{}({}))", axum_path, method, func_name);
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

    for cap in get_endpoint_metadata_regex().captures_iter(content) {
        metadata.insert(cap[1].to_string(), cap[2].to_string());
    }

    let relative_path_no_ext = path.strip_prefix(root_api_path).expect("Valid path");
    let relative_path_str = relative_path_no_ext.to_str().expect("Valid string");

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

fn inject_openapi_metadata(
    content: &str,
    metadata: &HashMap<String, String>,
    func_name: &str,
) -> Result<String> {
    let mut final_content = content.to_string();

    // 1. Manage utoipa imports based on actual need
    let has_structs = final_content.contains("pub struct ") || final_content.contains("pub enum ");
    let has_utoipa_import = final_content.contains("use utoipa::ToSchema;") || 
                             final_content.contains("use utoipa::{ToSchema") || 
                             final_content.contains("use utoipa::*");

    if has_structs && !has_utoipa_import {
        if let Some(pos) = final_content.find("use serde") {
            let end_of_line = final_content[pos..].find('\n').unwrap_or(0);
            final_content.insert_str(pos + end_of_line + 1, "use utoipa::ToSchema;\n");
        }
    } else if !has_structs && has_utoipa_import {
        // Remove unused import (brute force cleanup of past mess)
        final_content = final_content.replace("use utoipa::ToSchema;\n", "");
    } else {
        // Simple deduplication: if it appears twice, remove the first one
        let count = final_content.matches("use utoipa::ToSchema;").count();
        if count > 1 {
            final_content = final_content.replacen("use utoipa::ToSchema;\n", "", count - 1);
        }
    }

    // 1b. Ensure all structs derive ToSchema
    let struct_regex = Regex::new(r"(?m)^#\[derive\(([^)]*?(?:Serialize|Deserialize)[^)]*?)\)\]").unwrap();
    let mut updated_content = final_content.clone();
    for cap in struct_regex.captures_iter(&final_content) {
        let full_derive = cap.get(0).unwrap().as_str();
        let inner_derive = &cap[1];
        if !inner_derive.contains("ToSchema") {
            let new_derive = full_derive.replace(inner_derive, &format!("{}, ToSchema", inner_derive));
            updated_content = updated_content.replace(full_derive, &new_derive);
        }
    }
    final_content = updated_content;

    // 2. Prepare the utoipa::path macro
    let method = metadata.get("ENDPOINT_METHOD").cloned().unwrap_or_else(|| "get".to_string());
    let path = metadata.get("ENDPOINT_PATH").cloned().unwrap_or_else(|| "/".to_string());
    let description = metadata.get("ENDPOINT_DESCRIPTION").cloned().unwrap_or_else(|| "".to_string());
    let tag = metadata.get("ENDPOINT_TAG").cloned().unwrap_or_else(|| "api".to_string());
    let operation_id = metadata.get("OPERATION_ID").cloned().unwrap_or_else(|| func_name.to_string());
    let mut response_body = metadata.get("SUCCESS_RESPONSE_BODY").cloned().unwrap_or_else(|| "serde_json::Value".to_string());

    // Strip Json<...> or axum::Json<...> wrappers as utoipa wants the inner type
    if response_body.starts_with("Json<") && response_body.ends_with('>') {
        response_body = response_body[5..response_body.len()-1].to_string();
    } else if response_body.starts_with("axum::Json<") && response_body.ends_with('>') {
        response_body = response_body[11..response_body.len()-1].to_string();
    } else if response_body.starts_with("ApiResponse<") && response_body.ends_with('>') {
         // Keep ApiResponse<...> but it will require manual schema registration or specialized handling
    }

    // Standardize path for Utoipa: replace [param] with {param}
    let utoipa_path = get_dynamic_regex().replace_all(&path, "{$1}").to_string();

    let utoipa_macro = format!(
        r#"#[utoipa::path(
    {},
    path = "{}",
    tag = "{}",
    operation_id = "{}",
    responses(
        (status = 200, description = "{}", body = {}),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
"#,
        method, utoipa_path, tag, operation_id, description, response_body
    );

    // 3. Inject the macro above the function
    let fn_regex = Regex::new(&format!(r"(?m)^(\s*)pub async fn\s+{}\s*\(", func_name)).unwrap();
    
    // Check if macro already exists to avoid duplication
    // We check for utoipa::path and the specific operation_id
    if !final_content.contains("#[utoipa::path") || !final_content.contains(&format!("operation_id = \"{}\"", operation_id)) {
        if let Some(caps) = fn_regex.captures(&final_content) {
            let indent = &caps[1];
            let matched_text = caps.get(0).unwrap().as_str();
            
            // For existing macros, we might need a more complex replacement, but for simplicity:
            // if "#[utoipa::path" is already there but with different ID, we skip it for now to avoid mess.
            if !final_content.contains("#[utoipa::path") {
                let new_text = format!("{}{}{}", indent, utoipa_macro.replace("\n", &format!("\n{}", indent)), matched_text.trim_start());
                final_content = final_content.replace(matched_text, &new_text);
            }
        }
    }

    Ok(final_content)
}

fn extract_discovered_schemas(content: &str, module_path_prefix: &str, file_stem: &str) -> Vec<String> {
    let mut schemas = Vec::new();
    let struct_regex = Regex::new(r"(?m)^#\[derive\(.*?ToSchema.*?\)\]\s+pub (?:struct|enum)\s+([a-zA-Z0-9_]+)").unwrap();
    
    for cap in struct_regex.captures_iter(content) {
        let name = &cap[1];
        schemas.push(format!("{}::{}::{}", module_path_prefix, file_stem, name));
    }
    schemas
}
