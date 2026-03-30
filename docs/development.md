# Development & Maintenance Guide

This service is optimized for minimal resource usage. Developers must adhere to the following protocols to maintain the current architecture.

## 🏗 Protocols: Minimalist Implementation

- **No Suppression Flags**: **STRICTLY FORBIDDEN** from using `#[allow(...)]` or `@ts-ignore` to bypass the compiler or linter. Fix the underlying logic or types instead.
- **Dependency Control**: Before adding a new crate to `Cargo.toml`, evaluate if the functionality can be implemented natively or with an existing crate.
- **Dead Code Removal**: Standard protocol is to remove functions, types, and modules that are no longer used.

## 🛠 Adding Scrapers

When adding a new Anime or Komik source:

1. **Register Router**: Define the module and register it via `register_routes`. The `build.rs` script will auto-discover it for the routing system.
2. **MIME Verification**: Verify all image links returned using the `infer` crate if they are meant for proxying.
3. **Error Handling**: Wrap external requests in `reqwest` with appropriate timeouts and retry logic.

## 📦 Database & Connections

- **Pool Configuration**: SeaORM is configured for 20 max and 1 min connection. Adjust this in `src/bootstrap/mod.rs` only if metrics indicate connection exhaustion.
- **Code Generation**: Avoid excessive use of complex procedural macros that significantly increase compile time.

## 🚀 Performance Audit

Before merging changes:

1. **Verify `/metrics`**: For any negative impact on request latency.
2. **Binary Sizes**: Check for significant size increases.

## 🧹 Scheduled Maintenance

- **CDN Audit**: Execute `/api/proxy/image-cache/audit` periodically to identify and repair broken image URLs.
- **Cache Cleanup**: The `CleanupOldCache` task runs daily at 2 AM. Ensure it is working correctly to prevent data bloat in Redis and MySQL.
