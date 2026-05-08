/// Build script for generating API routes and OpenAPI documentation.
///
/// This script performs the following tasks:
/// - Creates the API routes directory if it doesn't exist
/// - Generates module files for the API routes
/// - Generates the root API module with OpenAPI schemas and handlers
/// - Validates OpenAPI specifications before writing
/// - Ensures rebuilds occur when build utilities or API routes change
///
/// The build process runs in two phases:
/// 1. Dry run: Validates everything without modifying files
/// 2. Actual build: Writes the generated code and specifications
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use itertools::Itertools;

mod build_utils;
use build_utils::{auto_mod_generator, mod_generator, openapi_generator, BuildOperation};

/// Configuration for the build process
#[derive(Debug)]
struct BuildConfig {
    api_routes_path: PathBuf,
    build_utils_path: PathBuf,
    enable_logging: bool,
    /// Root directory of the project (from CARGO_MANIFEST_DIR)
    project_root: PathBuf,
}

impl BuildConfig {
    fn from_env() -> Result<Self> {
        let project_root =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR not set")?);

        Ok(Self {
            api_routes_path: "src/presentation/api".into(),
            build_utils_path: "build_utils/".into(),
            enable_logging: env::var("VERBOSE").is_ok(),
            project_root,
        })
    }
}

/// Type alias for API handlers
type ApiHandlers = Vec<build_utils::handler_updater::HandlerRouteInfo>;

fn main() -> Result<()> {
    let config = BuildConfig::from_env()?;

    if config.enable_logging {
        env_logger::init();
        log::info!("🚀 Starting API build process");
        log::info!("📂 Project root: {}", config.project_root.display());
    }

    let mut operation = BuildOperation::new();

    // Setup environment once
    let api_routes_path = setup_build_environment(&config)?;

    // Use auto-routing by default (new system)
    let use_legacy = env::var("DISABLE_AUTO_ROUTING").is_ok();

    // Phase 1: Data Collection & Validation
    println!("cargo:warning=🔍 Phase 1: Validating API structure...");
    let (api_handlers, modules) = if use_legacy {
        println!("cargo:warning=⚠️  Using legacy routing system (DISABLE_AUTO_ROUTING=1)");
        collect_api_data(&api_routes_path)?
    } else {
        println!("cargo:warning=🚀 Using auto-routing system (default)");
        collect_api_data_auto(&api_routes_path)?
    };
    println!("cargo:warning=✓ Validation passed ({} handlers, {} modules)", api_handlers.len(), modules.len());

    // Phase 2: Build
    println!("cargo:warning=🔨 Phase 2: Building OpenAPI spec...");
    openapi_generator::generate_openapi_spec(&config.project_root, &api_handlers)?;

    // Store stats
    let routing_mode = if use_legacy { "legacy" } else { "auto" };
    operation.add_warning(format!(
        "API generated with {} handlers, {} modules ({})",
        api_handlers.len(),
        modules.len(),
        routing_mode
    ));

    // Print summary
    print_build_summary(&operation);

    println!("cargo:warning=✅ API build completed successfully");
    log::info!("✨ API build process completed successfully");
    Ok(())
}

/// Print a summary of the build process
fn print_build_summary(operation: &BuildOperation) {
    if operation.has_warnings() || operation.has_errors() {
        println!("cargo:warning=\n📊 Build Summary:");
        if operation.has_errors() {
            println!("cargo:warning=  ❌ Errors: {}", operation.errors.len());
            for error in &operation.errors {
                println!("cargo:warning=    - {}", error);
            }
        }
        if operation.has_warnings() {
            for warning in &operation.warnings {
                println!("cargo:warning=    ℹ️  {}", warning);
            }
        }
    }
}

fn setup_build_environment(config: &BuildConfig) -> Result<PathBuf> {
    log::debug!("Setting up build environment");
    configure_cargo_reruns(config);
    create_api_routes_directory(config)?;
    Ok(config.api_routes_path.clone())
}

fn configure_cargo_reruns(config: &BuildConfig) {
    println!("cargo:rerun-if-env-changed=FORCE_API_REGEN");
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}/",
        config.api_routes_path.display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        config.build_utils_path.display()
    );
}

fn create_api_routes_directory(config: &BuildConfig) -> Result<()> {
    fs::create_dir_all(&config.api_routes_path).with_context(|| {
        format!(
            "Failed to create API routes directory: {:?}",
            config.api_routes_path
        )
    })?;
    Ok(())
}

fn collect_api_data(api_routes_path: &Path) -> Result<(ApiHandlers, Vec<String>)> {
    log::debug!("Collecting API data");

    let mut api_handlers = Vec::new();
    let mut modules = Vec::new();

    mod_generator::generate_mod_for_directory(
        api_routes_path,
        api_routes_path,
        &mut api_handlers,
        &mut modules,
    )?;

    let modules = modules.into_iter().unique().sorted().collect();

    Ok((api_handlers, modules))
}

/// Collect API data using auto-routing system
fn collect_api_data_auto(
    api_routes_path: &Path,
) -> Result<(ApiHandlers, Vec<String>)> {
    log::debug!("📡 Collecting API data with auto-routing");

    let mut api_handlers = Vec::new();
    let mut modules = Vec::new();

    // Use auto mod generator instead of manual traversal
    auto_mod_generator::generate_mods_auto(
        api_routes_path,
        &mut api_handlers,
        &mut modules,
    )?;

    let modules = modules.into_iter().unique().sorted().collect();

    log::info!("✅ Auto-routing collected {} routes", api_handlers.len());

    Ok((api_handlers, modules))
}
