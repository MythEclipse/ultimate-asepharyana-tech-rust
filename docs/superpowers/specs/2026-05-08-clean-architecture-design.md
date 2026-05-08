# Design Spec: Clean-Modular Architecture Refactor

**Date**: 2026-05-08  
**Topic**: Refactor `apps/rust` from Hybrid to Clean-Modular Architecture.

## 1. Purpose
Standardize codebase structure for rigidity, maintainability, and clear separation of concerns (SOC) without violating the **Zero-Bloat Policy**.

## 2. Target Architecture
Moving from current structure to a three-tier modular design:

### A. Presentation Layer (`src/presentation/`)
- **API Handlers**: Pure Axum handlers.
- **DTOs**: Request/Response models for external communication.
- **Middleware**: Cross-cutting concerns (CORS, Metrics, Logging).

### B. Core Layer (`src/core/`) - The Domain
- **Models**: Pure data structures (Plain Rust Objects).
- **Repository Traits**: Abstract interfaces for data persistence.
- **Use Cases**: Orchestration of business logic (e.g., `ScrapeAnime`, `ProcessImage`).
- **Dependencies**: None (or minimal shared utils).

### C. Infrastructure Layer (`src/infra/`) - The Adapters
- **Repositories**: SeaORM & Redis implementations of Core traits.
- **Scrapers**: Site-specific parsing logic implementing Core scraping traits.
- **External Clients**: HTTP Client (reqwest), Browser Pool.

### D. Shared Layer (`src/shared/`)
- **Utils**: Low-level helpers (Date, JSON, String).
- **Config**: Application configuration.
- **Errors**: Centralized error handling.

## 3. Implementation Strategy
1. **Phase 1**: Scaffold new directory structure.
2. **Phase 2**: Migrate `models` and `entities` to `core/models` and `infra/repositories`.
3. **Phase 3**: Refactor `scraping` logic into `infra/scrapers` and define traits in `core`.
4. **Phase 4**: Move Axum handlers to `presentation/api` and update routing.
5. **Phase 5**: Cleanup `helpers` into `shared/utils`.

## 4. Constraints
- **Zero-Bloat**: No new heavy dependencies for the sake of abstraction.
- **Performance**: Maintain latency metrics as defined in `docs/development.md`.
- **SeaORM**: Entities stay in `infra/` to keep `core/` pure.

## 5. Success Criteria
- All tests pass.
- `/metrics` show no latency regression.
- Circular dependencies are eliminated.
- Folder structure matches this spec.
