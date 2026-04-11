# Task 3: HTML Parsing Entry Points Audit

## Objective
Audit all HTML parsing entry points in `apps/rust/src` that can reach html5ever foster-parenting warning from the `scraper` crate v0.25.0.

---

## Summary

**CORRECTED:** The codebase has **TWO distinct HTML parsing entry points** (not one):
1. **Primary centralized entry point**: `parse_html()` in `src/helpers/web/scraping.rs:34`
   - Used by 52+ call paths (intermediate parsers + most route handlers)
2. **Direct bypass entry point**: `parse_filter_page()` in `src/routes/api/anime2/filter.rs:200`
   - Directly calls `Html::parse_document(html)`, **bypassing parse_html()**

**Impact:**
- **1 primary parser function**: `parse_html()` at scraping.rs:34
- **1 direct bypass**: `parse_filter_page()` at filter.rs:200  
- **6 intermediate parsing helpers**: Functions in `scraping/anime2.rs` that call `parse_html()`
- **~44 route handlers**: HTTP endpoints (including /api/anime2/filter which triggers bypass)

All HTML fed to the parser comes from **untrusted upstream third-party sources** (alqanime.si, otakudesu.blog, komiku.org, api.komiku.org).

---

## Deliverable 1: Complete List of Parse Entry Points

### ⚠️ CRITICAL: TWO Parser Entry Points Identified

| Entry Point | File Path | Lines | Role | Risk |
|---|---|---|---|---|
| `parse_html(html: &str) -> Html` | `src/helpers/web/scraping.rs` | 34–35 | **Primary centralized** point where `Html::parse_document()` is called; used by intermediate parsers and most routes | Low - exists |
| `parse_filter_page(html, page)` | `src/routes/api/anime2/filter.rs` | 196–200 | **DIRECT BYPASS** – calls `Html::parse_document()` directly without going through `parse_html()`; triggered by `GET /api/anime2/filter` route | **HIGH** - uncontrolled |

**Implementation - Primary Entry Point:**
```rust
pub fn parse_html(html: &str) -> Html {
    Html::parse_document(html)  // Calls scraper crate, which uses html5ever
}
```

**Implementation - Bypass Entry Point:**
```rust
fn parse_filter_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<FilterAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = Html::parse_document(html);  // ← DIRECT CALL, bypasses parse_html()
    // ... parsing logic continues
}
```

### Level 2: Intermediate Parsing Functions (Use Primary Entry Point)

All in `src/scraping/anime2.rs` — each correctly calls `parse_html()` directly:

| Function | Lines | Called From | Description |
|---|---|---|---|
| `parse_ongoing_anime()` | 75–105 | Route: `/api/anime2/ongoing_anime/{slug}` via `parse_ongoing_anime_document()` | Parses ongoing anime listing HTML |
| `parse_ongoing_anime_with_score()` | 107–136 | Routes: anime2 ongoing routes | Extends parse_ongoing_anime with score extraction |
| `parse_complete_anime()` | 138–168 | Route: `/api/anime2/complete_anime/{slug}` | Parses completed anime listing HTML |
| `parse_latest_anime()` | 170–201 | Route: `/api/anime2/latest/{slug}` | Parses latest anime listings |
| `parse_search_anime()` | 203–241 | Route: `/api/anime2/search/{slug}/{page}` | Parses anime search results |
| `parse_genre_anime()` | 243–278 | Route: `/api/anime2/genre/{slug}/{page}` | Parses genre-filtered anime listings |
| `parse_pagination_with_string()` | 315–... | Various routes | Pagination extractor; consumes parsed Html document (does not parse HTML) |

**Call pattern in parsing functions:** Each `parse_xxx` function (except pagination extractors) has signature `fn parse_xxx(html: &str) -> Result<Vec<T>, ...>` and always starts with:
```rust
let document = parse_html(html);
```

**Exception:** `parse_pagination_with_string()` and `parse_pagination()` are extractors that take an already-parsed `&Html` document; they do **not** parse HTML themselves nor call `parse_html()`.

---

## Deliverable 2: Route-to-Parser Call Graph

### Mapping: Routes → Intermediate Parsers → `parse_html()`

#### Anime2 Routes (fetch from `https://alqanime.si/anime/...`)

**Filter Endpoint (BYPASS - Direct Entry Point 2):**
- Route: `GET /api/anime2/filter`
- File: `src/routes/api/anime2/filter.rs:167–191`
- Handler: `filter()` → `fetch_filtered_anime()` → `parse_filter_page()` → **`Html::parse_document(html)` (LINE 200, DIRECT CALL)**
- **Status**: ⚠️ **CRITICAL BYPASS** – does NOT use `parse_html()` helper
- **Impact**: This route's HTML parsing is NOT controlled by the primary entry point

**Latest Endpoint:**
- Route: `GET /api/anime2/latest/{slug}` 
- File: `src/routes/api/anime2/latest/[slug].rs:124`
- Handler: `latest()` → `fetch_latest_anime()` → `parse_latest_page()` → **`parse_html(html)`** (line 124)
- Then calls: `parsers::parse_latest_anime(html)` → internally calls `parse_html()` again (line 170 in anime2.rs)
- **Note:** `parse_html()` is called twice: once in `parse_latest_page()`, once inside `parse_latest_anime()`

**Ongoing Anime Endpoint:**
- Route: `GET /api/anime2/ongoing_anime/{slug}`
- File: `src/routes/api/anime2/ongoing_anime/[slug].rs:142`
- Handler: `slug()` → `fetch_ongoing_anime_page()` → `parse_ongoing_anime_document()` → **`parse_html(html)`** (line 142)
- Then calls: `parsers::parse_ongoing_anime_with_score(html)` → internally calls `parse_html()` (line 110 in anime2.rs)

**Complete Anime Endpoint:**
- Route: `GET /api/anime2/complete_anime/{slug}`
- File: `src/routes/api/anime2/complete_anime/[slug].rs:145`
- Handler: calls `parsers::parse_complete_anime(html)` which calls `parse_html()` at line 141

**Search Endpoint:**
- Route: `GET /api/anime2/search/{slug}/{page}`
- File: `src/routes/api/anime2/search/[slug]/[page].rs:128`

**Genre Endpoint:**
- Route: `GET /api/anime2/genre/{slug}/{page}`
- File: `src/routes/api/anime2/genre/[slug]/[page].rs:292`

**Genre List Endpoint:**
- Route: `GET /api/anime2/genre_list`
- File: `src/routes/api/anime2/genre_list.rs:98`

#### Anime Routes (fetch from `https://otakudesu.blog/...`)

**Latest Endpoint:**
- Route: `GET /api/anime/latest/{slug}`
- File: `src/routes/api/anime/latest/[slug].rs:170`
- Calls: `crate::helpers::scraping::parse_html(html)` directly

**Ongoing Anime:**
- File: `src/routes/api/anime/ongoing_anime/[slug].rs:158`

**Genre Routes:**
- Files: 
  - `src/routes/api/anime/genre/[slug]/[page].rs:205`
  - `src/routes/api/anime/genre/[slug]/index.rs:203`

**Search Routes:**
- Files:
  - `src/routes/api/anime/search/[slug]/index.rs:188`
  - `src/routes/api/anime/search/[slug]/[page].rs:196`

**Detail, Complete, Full, Index Routes:**
- `src/routes/api/anime/detail/[slug].rs:261`
- `src/routes/api/anime/complete_anime/[slug].rs:144`
- `src/routes/api/anime/full/[slug].rs:200`
- `src/routes/api/anime/index.rs:171, 208`

**Genre List:**
- `src/routes/api/anime/genre_list.rs:95`

#### Komik Routes (fetch from `https://komiku.org/...` or `https://api.komiku.org/...`)

**Manga Routes:**
- `src/routes/api/komik/manga/[slug].rs:176`
- `src/routes/api/komik/manhwa/[slug].rs:170`
- `src/routes/api/komik/manhua/[slug].rs:172`

**Chapter Route:**
- `src/routes/api/komik/chapter/[slug].rs:137`

**Genre Routes:**
- `src/routes/api/komik/genre/[slug]/index.rs:220`
- `src/routes/api/komik/genre/[slug]/[page].rs:220`

**Popular Route:**
- `src/routes/api/komik/popular/[slug].rs:204`

**Search Routes:**
- `src/routes/api/komik/search/[slug]/index.rs:207`
- `src/routes/api/komik/search/[slug]/[page].rs:207`

**Detail & Genre List:**
- `src/routes/api/komik/detail/[slug].rs:214`
- `src/routes/api/komik/genre_list.rs:94`

#### Scraping Module (non-route context)

Test/demonstration binaries that call `parse_html()`:
- `src/bin/capture_warning.rs:54`
- `src/bin/foster_parenting_assertion.rs:37, 64, 93` (test code, not production)

---

## Deliverable 3: Untrusted HTML Flagging

### All Paths Parse Untrusted/Malformed Upstream HTML

| Source | Status | Notes |
|---|---|---|
| **alqanime.si** | ⚠️ **UNTRUSTED** | Third-party anime listing site; can serve malformed HTML; no control over source |
| **otakudesu.blog** | ⚠️ **UNTRUSTED** | Third-party blog; can serve malformed HTML; no control over source |
| **komiku.org** | ⚠️ **UNTRUSTED** | Third-party manga site; can serve malformed HTML; no control over source |
| **api.komiku.org** | ⚠️ **UNTRUSTED** | Third-party API; can serve malformed HTML; no control over source |

### Critical: Double-Parsing Issue

**Many routes call `parse_html()` TWICE on the same HTML:**

Example from `src/routes/api/anime2/latest/[slug].rs:124`:
```rust
fn parse_latest_page(html: &str, current_page: u32) -> Result<...> {
    let document = crate::helpers::parse_html(html);  // ← FIRST CALL (line 124)
    let anime_list = parsers::parse_latest_anime(html)?;  // ← SECOND PARSE inside this function (line 127)
    let pagination = parsers::parse_pagination(&document, current_page);  // Reuses first parse
    Ok((anime_list, pagination))
}
```

Inside `parsers::parse_latest_anime(html)` at `anime2.rs:170`:
```rust
pub fn parse_latest_anime(html: &str) -> Result<...> {
    let document = parse_html(html);  // ← SECOND CALL, parses HTML again!
    // ... extraction logic
}
```

**Impact:** Same untrusted HTML triggers html5ever twice, **doubling the window for foster-parenting warning**.

---

## Deliverable 4: Corrected Safe Fix Boundaries and Strategy

### ⚠️ AUDIT FINDING: Entry Point Bypass Requires Dual Boundary Fix

**Problem:** The audit's original claim of a single entry point is **INCORRECT**. 

- **Entry Point 1 (Primary)**: `parse_html()` in `src/helpers/web/scraping.rs:34`
  - Protects: 52+ call paths across intermediate parsers and most routes
  
- **Entry Point 2 (Bypass)**: `parse_filter_page()` in `src/routes/api/anime2/filter.rs:200`
  - Protects: `/api/anime2/filter` route ONLY
  - **Not controlled by primary entry point**

### Option A: Dual-Boundary Fix (Narrowest Safe Scope)

**Two separate fixes required:**

1. **Fix Location 1**: `src/helpers/web/scraping.rs::parse_html()` (lines 34–35)
   - Impact: 52+ paths via intermediate parsers
   - Implementation: Silence warning or normalize HTML
   - Risk: Low

2. **Fix Location 2**: `src/routes/api/anime2/filter.rs::parse_filter_page()` (line 200)
   - Impact: `/api/anime2/filter` route only
   - Implementation: Either:
     - **Option A1**: Change direct `Html::parse_document()` call to use `parse_html()` helper
       ```rust
       // BEFORE (line 200):
       let document = Html::parse_document(html);
       
       // AFTER:
       let document = crate::helpers::parse_html(html);
       ```
     - **Option A2**: Apply same fix logic (warning silence or normalization) locally
   - Risk: Low if using Option A1 (consolidation); Low-Medium if using Option A2 (duplication)

**Acceptance Criteria for Dual-Boundary:**
1. ✅ Primary fix applied to `parse_html()` in `src/helpers/web/scraping.rs:34`
2. ✅ Secondary fix applied to `parse_filter_page()` in `src/routes/api/anime2/filter.rs:200`
3. ✅ Both fixes ensure html5ever warning is silenced for all 53+ call paths
4. ✅ No functional behavior change
5. ✅ All routes including `/api/anime2/filter` pass existing tests

---

### Option B: Consolidation-First Strategy (Recommended)

**Task 4: Consolidation Phase** (must execute before applying Task 4/5 fixes)
- Refactor `parse_filter_page()` to call `parse_html()` instead of `Html::parse_document()` directly
- File: `src/routes/api/anime2/filter.rs:200`
- Change: `let document = Html::parse_document(html)` → `let document = crate::helpers::parse_html(html)`
- Benefit: Creates true single entry point before applying warning fix

**Task 5: Single-Boundary Fix** (executes after consolidation)
- Apply warning silence or HTML normalization to `parse_html()` ONLY
- File: `src/helpers/web/scraping.rs:34`
- Benefit: Single, minimal fix; no duplication; all 53+ paths protected by one change

**Acceptance Criteria for Consolidation-First:**
1. ✅ Bypass removed: `parse_filter_page()` now uses `crate::helpers::parse_html(html)`
2. ✅ Single entry point established and verified
3. ✅ Fix applied to only `parse_html()` in `src/helpers/web/scraping.rs:34`
4. ✅ All routes including `/api/anime2/filter` pass existing tests
5. ✅ No functional behavior change

---

### Recommended Path Forward

**Option B (Consolidation-First) is STRONGLY RECOMMENDED:**
- Reduces complexity: one fix location vs two
- Eliminates duplication: no repeated fix logic
- Improves maintainability: single source of truth
- Better long-term: prevents future bypasses

**If Option A (Dual-Boundary) is chosen:**
- Use A1 (consolidation within Option A) rather than A2 to minimize duplication
- Must verify both call sites are covered in testing

### Why NOT to Fix at Route Level

❌ **Anti-pattern:** Fixing at each route handler (44 locations)
- 44 separate edits needed
- Risk of missing locations
- Maintenance burden
- Violates DRY principle

❌ **Anti-pattern:** Fixing in intermediate parsers (6 locations)
- Still too many locations
- Duplicates logic
- Future parsers added might bypass fix

### Fix Strategy: Disable Foster Parenting Warning

The html5ever parser emits the warning when it encounters malformed HTML that requires tree reconstruction. The warning is generated by the scraper crate's dependency chain at runtime. **Rust lint attributes (`#[allow(...)]`) cannot silence runtime warnings from external crates.**

**Option 1: Suppress html5ever Warnings via EnvFilter (Runtime Logging Configuration)**

The application uses `tracing_subscriber::EnvFilter` configured in `src/bootstrap/mod.rs` (lines 27–32):
```rust
let env_filter = match std::env::var("RUST_LOG") {
    Ok(filter) => EnvFilter::new(filter),
    Err(_) => EnvFilter::new("warn"),
};
```

To suppress html5ever foster-parenting warnings, set `RUST_LOG` environment variable at application startup:
```bash
# Suppress html5ever debug logs while maintaining app warnings
export RUST_LOG="warn,html5ever=error"
```

This can also be configured at application bootstrap without requiring external env vars:
```rust
// In src/bootstrap/mod.rs around line 28, if want to suppress html5ever unconditionally:
let env_filter = match std::env::var("RUST_LOG") {
    Ok(filter) => {
        // User-provided filter; suppress html5ever debug logs automatically
        format!("{},html5ever=error", filter)
    },
    Err(_) => "warn,html5ever=error".to_string(),
};
let env_filter = EnvFilter::new(&env_filter);
```

- Scope: Application startup configuration (no code change required, or minimal bootstrap change)
- Risk: Low (respects `RUST_LOG` env var convention)
- Behavior change: None (diagnostic output only)
- **Benefit:** Non-invasive, leverages existing logging infrastructure

**Option 2: Normalize HTML Before Parsing (Technically Valid)**
```rust
pub fn parse_html(html: &str) -> Html {
    let normalized = normalize_html_for_parser(html);
    Html::parse_document(&normalized)
}
```
- Scope: Single function + normalization helper
- Risk: Low (normalization is passive)
- Behavior change: Possible (if normalization alters output)
- **Benefit:** Addresses root cause (malformed HTML); solves warning at source

**Option 3: Update html5ever Dependency (Upstream Fix)**
- Requires changes to scraper crate or html5ever upstream
- Out of scope for current narrow fix
- **Best long-term:** File issue upstream to make warning suppressible or provide log-level control

**IMPORTANT NOTE:** Rust lint attributes like `#[allow(invalid_html5_parsing)]` are **invalid** and do not work with runtime warnings from external crates. Intercepting stderr via nonexistent APIs like `io::stderr::set_write()` is also invalid. Only Options 1 or 2 are technically correct and production-safe.

---

## Entry Point Call Flows

### Flow 1: Primary Entry Point (52+ paths)
```
HTTP Route Handler (most routes)
  ↓
fetch_html_with_retry(url)  [fetches from alqanime.si / otakudesu.blog / komiku.org]
  ↓
Intermediate Parser [parse_latest_anime, parse_ongoing_anime, etc.]
  ↓
parse_html(html)  ← **PRIMARY ENTRY POINT** (src/helpers/web/scraping.rs:34)
  ↓
Html::parse_document(html)  [scraper crate]
  ↓
html5ever::tree_builder  [EMITS FOSTER-PARENTING WARNING]
```

### Flow 2: Bypass Entry Point (/api/anime2/filter ONLY)
```
GET /api/anime2/filter
  ↓
filter() handler
  ↓
fetch_filtered_anime(page, genre, status, type, order)
  ↓
parse_filter_page(html, page)
  ↓
Html::parse_document(html)  ← **BYPASS ENTRY POINT** (src/routes/api/anime2/filter.rs:200)
  ↓
html5ever::tree_builder  [EMITS FOSTER-PARENTING WARNING]
```

---

## Files Involved (44 Route Handlers + 2 Parsing Entry Points + 6 Intermediate Parsers)

### Parsing Entry Points (2 files):
- **Primary**: `src/helpers/web/scraping.rs:34` – `parse_html()`
- **Bypass**: `src/routes/api/anime2/filter.rs:200` – `parse_filter_page()`

### Intermediate Parsers (1 file):
- `src/scraping/anime2.rs` – 6 parsing functions that use primary entry point

### Route Handlers (44 files):
- `src/routes/api/anime2/filter.rs` – ⚠️ Uses bypass entry point
- `src/routes/api/anime2/latest/[slug].rs`
- `src/routes/api/anime2/ongoing_anime/[slug].rs`
- `src/routes/api/anime2/complete_anime/[slug].rs`
- `src/routes/api/anime2/search/[slug]/[page].rs`
- `src/routes/api/anime2/search/[slug]/index.rs`
- `src/routes/api/anime2/genre/[slug]/[page].rs`
- `src/routes/api/anime2/genre/[slug]/index.rs`
- `src/routes/api/anime2/genre_list.rs`
- `src/routes/api/anime/latest/[slug].rs`
- `src/routes/api/anime/ongoing_anime/[slug].rs`
- `src/routes/api/anime/genre/[slug]/[page].rs`
- `src/routes/api/anime/genre/[slug]/index.rs`
- `src/routes/api/anime/search/[slug]/index.rs`
- `src/routes/api/anime/search/[slug]/[page].rs`
- `src/routes/api/anime/detail/[slug].rs`
- `src/routes/api/anime/complete_anime/[slug].rs`
- `src/routes/api/anime/full/[slug].rs`
- `src/routes/api/anime/index.rs`
- `src/routes/api/anime/genre_list.rs`
- `src/routes/api/komik/manga/[slug].rs`
- `src/routes/api/komik/manhwa/[slug].rs`
- `src/routes/api/komik/manhua/[slug].rs`
- `src/routes/api/komik/chapter/[slug].rs`
- `src/routes/api/komik/genre/[slug]/index.rs`
- `src/routes/api/komik/genre/[slug]/[page].rs`
- `src/routes/api/komik/popular/[slug].rs`
- `src/routes/api/komik/search/[slug]/index.rs`
- `src/routes/api/komik/search/[slug]/[page].rs`
- `src/routes/api/komik/detail/[slug].rs`
- `src/routes/api/komik/genre_list.rs`
- Plus 13+ more anime/komik detail & listing routes

## Corrected Conclusion

**AUDIT FINDING: Dual Entry Points Identified**

The codebase has **TWO** distinct HTML parsing entry points:

1. **Primary Entry Point** - `parse_html()` at `src/helpers/web/scraping.rs:34–35`
   - Used by: 52+ paths through intermediate parsers and most routes
   
2. **Bypass Entry Point** - `parse_filter_page()` at `src/routes/api/anime2/filter.rs:200`
   - Used by: `/api/anime2/filter` route ONLY
   - Risk: Direct `Html::parse_document()` call not controlled by primary entry point

**Previous Audit Claim: INCORRECT**
- ❌ "The codebase has a single centralized HTML parsing entry point"
- ❌ "No other parsing entry points exist"

**Corrected Assessment:**
- ✅ Primary entry point exists at `parse_html()` 
- ✅ Bypass entry point exists at `parse_filter_page()` 
- ✅ Total 53+ reachable call paths via 2 distinct HTML parsing locations

---

### Recommended Fix Strategy for Tasks 4/5

**Strategy A (Dual-Boundary):**
- Fix both entry points separately
- Primary: `src/helpers/web/scraping.rs:34` – silence warning
- Secondary: `src/routes/api/anime2/filter.rs:200` – silence warning or consolidate to primary
- ✅ Works but introduces duplication if both silenced independently

**Strategy B (Consolidation-First, RECOMMENDED):**
- **Phase 1**: Consolidate bypass → use primary entry point
  - Change `parse_filter_page()` line 200 to call `crate::helpers::parse_html(html)` instead
- **Phase 2**: Fix primary entry point only
  - Apply warning silence or normalization to `parse_html()` once
  - All 53+ paths automatically protected
- ✅ Single fix location, minimal code, future-proof, DRY compliant

**Acceptance Criteria (both strategies):**
1. ✅ All 53+ HTML parsing paths covered by fix
2. ✅ `/api/anime2/filter` route tested and working
3. ✅ No functional behavior change
4. ✅ Existing test suites pass
5. ✅ html5ever foster-parenting warning silenced for all untrusted upstream HTML
