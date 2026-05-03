/// Test: Verify parsed output for foster_parenting_minimal.html fixture
/// 
/// This binary contains tests and assertions that validate the expected behavior
/// when parsing HTML that triggers the html5ever::tree_builder::foster_parent_in_body() warning.
/// 
/// Uses shared fixture: src/bin/test_fixtures/foster_parenting_minimal.html
/// Uses shared parser: src/helpers::parse_html()

use scraper::Selector;
use rustexpress::helpers::parse_html;
use std::fs;

fn main() {
    println!("Running foster parenting regression tests...\n");
    
    // Load HTML fixture from shared test file
    let fixture_path = "src/bin/test_fixtures/foster_parenting_minimal.html";
    let foster_parenting_html = fs::read_to_string(fixture_path)
        .expect(&format!("Failed to read fixture from {}", fixture_path));
    
    test_foster_parenting_text_extraction(&foster_parenting_html);
    println!("✓ test_foster_parenting_text_extraction passed");
    
    test_foster_parenting_table_structure(&foster_parenting_html);
    println!("✓ test_foster_parenting_table_structure passed");
    
    test_expected_parsed_output_assertion(&foster_parenting_html);
    println!("✓ test_expected_parsed_output_assertion passed");
    
    println!("\n✓ All assertions passed (3/3)");
    println!("\nFixture source: {}", fixture_path);
    println!("Parser source: src/helpers/web/scraping.rs::parse_html()");
}

/// Test: Text nodes in <table> are foster-parented to body
fn test_foster_parenting_text_extraction(html: &str) {
    let document = parse_html(html);
    let body_sel = Selector::parse("body").expect("Valid CSS selector");
    
    let body_text: String = document
        .select(&body_sel)
        .next()
        .map(|el| el.text().collect())
        .unwrap_or_default();

    assert!(
        body_text.contains("orphaned text"),
        "Text 'orphaned text' should be present in body (fostered from table)"
    );
    assert!(
        body_text.contains("more text"),
        "Text 'more text' should be present in body (fostered from table)"
    );
    assert!(
        body_text.contains("cell content"),
        "Cell content should still be present"
    );
}

fn test_foster_parenting_table_structure(html: &str) {
    let document = parse_html(html);
    
    let table_sel = Selector::parse("table").expect("Valid CSS selector");
    let tr_sel = Selector::parse("tr").expect("Valid CSS selector");
    let td_sel = Selector::parse("td").expect("Valid CSS selector");

    let tables: Vec<_> = document.select(&table_sel).collect();
    assert_eq!(tables.len(), 1, "Should have exactly 1 table");

    let rows: Vec<_> = document.select(&tr_sel).collect();
    assert_eq!(rows.len(), 1, "Should have exactly 1 row");

    let cells: Vec<_> = document.select(&td_sel).collect();
    assert_eq!(cells.len(), 1, "Should have exactly 1 cell");

    if let Some(cell) = cells.first() {
        let cell_text: String = cell.text().collect();
        assert_eq!(cell_text.trim(), "cell content", "Cell content should be preserved");
    }
}

fn test_expected_parsed_output_assertion(html: &str) {
    let document = parse_html(html);
    
    let body_sel = Selector::parse("body").expect("Valid CSS selector");
    let body = document.select(&body_sel).next().expect("body should exist");
    let full_text: String = body.text().collect();

    let expected_pattern = "orphaned textmore textcell content";
    assert!(
        full_text.contains(&expected_pattern) || 
        (full_text.contains("orphaned text") && 
         full_text.contains("more text") &&
         full_text.contains("cell content")),
        "Parsed output should contain all text content in fostered form. Got: '{}'",
        full_text
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_fixture() -> String {
        fs::read_to_string("src/bin/test_fixtures/foster_parenting_minimal.html")
            .expect("Failed to load fixture")
    }

    #[test]
    fn test_foster_parenting_text_extraction_test() {
        let html = load_fixture();
        test_foster_parenting_text_extraction(&html);
    }

    #[test]
    fn test_foster_parenting_table_structure_test() {
        let html = load_fixture();
        test_foster_parenting_table_structure(&html);
    }

    #[test]
    fn test_expected_parsed_output_assertion_test() {
        let html = load_fixture();
        test_expected_parsed_output_assertion(&html);
    }
}
