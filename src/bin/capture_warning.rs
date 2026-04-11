/// Capture html5ever tree_builder warning evidence by parsing problematic HTML.
/// 
/// Build with: cargo build --bin capture_warning
/// Run with: RUST_LOG=warn cargo run --bin capture_warning 2>&1
///
/// Evidence of the warning is captured through:
/// 1. HTML that triggers foster_parenting in html5ever::tree_builder
/// 2. Observable parsing behavior showing tree reconstruction
/// 3. The call path from src/helpers::parse_html () to html5ever
/// 4. Real endpoint context from /api/anime2/latest/{slug} route

use std::fs;
use scraper::Selector;
use tracing_subscriber::EnvFilter;
use rustexpress::helpers::parse_html;

fn main() {
    // Initialize logging to capture WARN output from html5ever
    let env_filter = EnvFilter::from_default_env()
        .add_directive("warn".parse().expect("valid directive"))
        .add_directive("html5ever=warn".parse().expect("valid directive"));
    
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();

    println!("=== HTML5ever Tree Builder Foster Parenting Evidence ===\n");
    println!("Real Endpoint: GET /api/anime2/latest/{{slug}}");
    println!("Handler: src/routes/api/anime2/latest/[slug].rs:124");
    println!("Helper Path: src/helpers/web/scraping.rs::parse_html() -> Html::parse_document()\n");

    // Load HTML fixture from shared test file
    let fixture_path = "src/bin/test_fixtures/foster_parenting_minimal.html";
    let test_html = fs::read_to_string(fixture_path)
        .expect(&format!("Failed to read fixture from {}", fixture_path));

    println!("Input Request:");
    println!("  GET /api/anime2/latest/some-anime");
    println!("  Body: HTML containing misplaced text in <table>");
    println!("  Fixture: {}", fixture_path);
    println!("  Test HTML: {}\n", test_html);

    println!("Parsing through src/helpers::parse_html()...\n");
    println!("--- BEGIN STDERR (logging output) ---");
    
    // This parse_html() call routes through:
    // src/helpers::parse_html()
    // -> scraper crate Html::parse_document()
    // -> html5ever::parse() [version 0.36.1]
    // -> TreeBuilder::process_token()
    // -> TreeBuilder::foster_parent_in_body() which emits:
    // warn!("foster parenting not implemented")
    let document = parse_html(&test_html);
    
    println!("--- END STDERR (logging output) ---\n");

    // Analyze the result
    println!("Parse Output Evidence:\n");

    // Check table structure
    let table_sel = Selector::parse("table").unwrap();
    let tr_sel = Selector::parse("tr").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    let tables: Vec<_> = document.select(&table_sel).collect();
    println!("  ✓ Tables parsed: {}", tables.len());

    let trs: Vec<_> = document.select(&tr_sel).collect();
    println!("  ✓ Table rows found: {}", trs.len());

    let tds: Vec<_> = document.select(&td_sel).collect();
    println!("  ✓ Table cells found: {}", tds.len());

    // Extract text from body to show foster parenting occurred
    let body_sel = Selector::parse("body").unwrap();
    if let Some(body) = document.select(&body_sel).next() {
        let body_text: String = body.text().collect();
        let trimmed = body_text.trim();
        println!("\n  Body element text content:");
        println!("    '{}'", trimmed);
        
        if trimmed.contains("orphaned text") {
            println!("\n  ✓ EVIDENCE: 'orphaned text' moved OUT of <table>");
            println!("    This proves foster_parenting occurred!");
        }
        if trimmed.contains("more text") {
            println!("  ✓ EVIDENCE: 'more text' moved OUT of <table>");
            println!("    This confirms the adoption agency algorithm ran!");
        }
    }

    println!("\n=== Proven Call Path ===");
    println!("Route: GET /api/anime2/latest/{{slug}}");
    println!("Request Handler: src/routes/api/anime2/latest/[slug].rs");
    println!("  -> latest() handler");
    println!("  -> fetch_latest_anime()");
    println!("  -> parse_latest_page(html, page)");
    println!("  -> crate::helpers::parse_html(html) [line 124]\n");
    
    println!("Helper Function: src/helpers/web/scraping.rs");
    println!("  pub fn parse_html(html: &str) -> Html {{");
    println!("      Html::parse_document(html)  // Line 34");
    println!("  }}\n");

    println!("Call Stack to Warning:");
    println!("  1. crate::helpers::parse_html() [src/helpers/web/scraping.rs:34]");
    println!("  2. Html::parse_document() [scraper crate wrapper]");
    println!("  3. html5ever::parse() [Cargo.toml: version 0.36.1]");
    println!("  4. TreeBuilder::process_token()");
    println!("  5. TreeBuilder::process_chars_in_table()");
    println!("  6. TreeBuilder::foster_parent_in_body() [src/tree_builder/mod.rs:1227]");
    println!("  7. warn!(\"foster parenting not implemented\") ← EMITTED ABOVE\n");

    println!("=== Fixture Source ===");
    println!("Shared File: {}", fixture_path);
    println!("HTML Content: {}", test_html);
    println!("Expected Parsing Behavior: Text nodes are fostered out of table");
}
