# Clean-Modular Architecture Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the codebase into a rigid Clean-Modular architecture to improve maintainability and strictly enforce separation of concerns.

**Architecture:** A three-tier modular approach consisting of Presentation (API Handlers/DTOs), Core (Domain Models/Traits/Use Cases), and Infrastructure (Adapters/Repositories/Scrapers).

**Tech Stack:** Rust, Axum, SeaORM, Redis, reqwest.

---

### Task 1: Initialize Core Domain Models & Shared Errors

**Files:**
- Create: `src/shared/errors/mod.rs`
- Create: `src/core/models/image.rs`
- Create: `src/core/models/mod.rs`
- Create: `src/shared/mod.rs`

- [ ] **Step 1: Define shared application errors**
- [ ] **Step 2: Define pure domain models for ImageCache**
- [ ] **Step 3: Setup core and shared modules in `lib.rs`**

```rust
// src/shared/errors/mod.rs
use axum::{response::{IntoResponse, Response}, Json, http::StatusCode};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
            AppError::Validation(m) => (StatusCode::BAD_REQUEST, m),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

- [ ] **Step 4: Commit changes**
```bash
git add src/shared/errors/mod.rs src/core/models/image.rs
git commit -m "feat: init core models and shared errors"
```

### Task 2: Define Core Repository Traits

**Files:**
- Create: `src/core/repositories/image_repository.rs`
- Create: `src/core/repositories/mod.rs`

- [ ] **Step 1: Define ImageRepository trait in `core`**

```rust
// src/core/repositories/image_repository.rs
use async_trait::async_trait;
use crate::core::models::image::ImageCache;
use crate::shared::errors::AppError;

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn find_by_original_url(&self, url: &str) -> Result<Option<ImageCache>, AppError>;
    async fn save(&self, image: ImageCache) -> Result<(), AppError>;
    async fn delete_by_original_url(&self, url: &str) -> Result<(), AppError>;
}
```

- [ ] **Step 2: Commit changes**
```bash
git add src/core/repositories/
git commit -m "feat: define core repository traits"
```

### Task 3: Migrate SeaORM Entities to Infrastructure

**Files:**
- Modify: `src/infra/mod.rs`
- Create: `src/infra/repositories/mysql_image_repository.rs`

- [ ] **Step 1: Implement ImageRepository for MySQL using SeaORM**
- [ ] **Step 2: Move `src/entities/image_cache.rs` logic into the new repository implementation**
- [ ] **Step 3: Update `src/infra/mod.rs` to expose repositories**

- [ ] **Step 4: Commit changes**
```bash
git add src/infra/repositories/
git commit -m "feat: implement mysql image repository in infra"
```

### Task 4: Implement Core Use Cases (Image Caching)

**Files:**
- Create: `src/core/use_cases/cache_image.rs`
- Create: `src/core/use_cases/mod.rs`

- [ ] **Step 1: Implement `CacheImageUseCase`**
- [ ] **Step 2: Orchestrate logic between repository, redis, and scraper/uploader**

- [ ] **Step 3: Commit changes**
```bash
git add src/core/use_cases/
git commit -m "feat: implement image caching use cases"
```

### Task 5: Refactor Scrapers into Infrastructure

**Files:**
- Create: `src/core/repositories/scraping_repository.rs`
- Create: `src/infra/scrapers/otakudesu.rs`

- [ ] **Step 1: Define Scraping traits in `core`**
- [ ] **Step 2: Implement site-specific scrapers in `infra`**
- [ ] **Step 3: Migrate existing logic from `src/scraping/`**

- [ ] **Step 4: Commit changes**
```bash
git add src/infra/scrapers/
git commit -m "feat: migrate scrapers to infra adapters"
```

### Task 6: Refactor Presentation Layer (API Handlers)

**Files:**
- Create: `src/presentation/api/anime_handler.rs`
- Create: `src/presentation/api/mod.rs`
- Create: `src/presentation/mod.rs`

- [ ] **Step 1: Migrate handlers from `src/routes/` to `presentation/api/`**
- [ ] **Step 2: Update handlers to use Use Cases instead of direct service/helper calls**
- [ ] **Step 3: Update global router in `src/main.rs` or `src/lib.rs`**

- [ ] **Step 4: Commit changes**
```bash
git add src/presentation/api/
git commit -m "feat: refactor presentation layer api handlers"
```

### Task 7: Global Cleanup & Verification

**Files:**
- Modify: `src/lib.rs`
- Delete: `src/helpers/` (partially merged into shared/infra)
- Delete: `src/services/` (merged into core/use_cases)
- Delete: `src/routes/` (merged into presentation)

- [ ] **Step 1: Update `lib.rs` to reflect new module structure**
- [ ] **Step 2: Remove old redundant directories**
- [ ] **Step 3: Run full test suite**
- [ ] **Step 4: Verify metrics endpoint**

- [ ] **Step 5: Final Commit**
```bash
git commit -m "refactor: complete clean-modular architecture overhaul"
```
