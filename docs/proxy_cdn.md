# Image Proxy & CDN Caching Architecture

RustExpress implements a multi-layer strategy for image delivery and storage.

## 🏗 Storage Strategy: External CDN

Instead of local storage, this service uses **Picser** to manage image persistence.

- **Picser Integration**: Images are uploaded to a GitHub repository (`picser`) via an internal API.
- **Permanent Links**: Once uploaded, images are accessible via JSDelivr or Statically CDN links.
- **Resource Management**: Managed external storage reducing local disk dependency.

## 🏎 Layered Caching

1. **Redis Cache (L1)**:
   - Maps `Original URL -> CDN URL`.
   - TTL: 7 days.
   - Purpose: Fast access for frequent requests.
2. **Database Cache (L2)**:
   - Persistent MySQL record in `image_cache` table.
   - Purpose: Permanent registry of successful uploads.
   - Includes metadata: `content_type`, `size`.

## 🛠 Proxy Flow

1. **Cache Lookup**: Redis, then DB.
2. **Request Coalescing**: Concurrent requests for the same URL wait for a single upload via `DashMap`.
3. **MIME Verification**: Downloads and verifies content using the `infer` crate.
4. **Concurrency Limit**: Uses a `Semaphore` to limit simultaneous uploads.
5. **CDN Upload**: Persists via Picser and returns the CDN URL.

## 🔂 Asset Audit & Repair

The endpoint `/api/proxy/image-cache/audit` provides programmed health checks:

- **Status Check**: Verifies CDN URL accessibility.
- **Binary Verification**: Performs a MIME signature check.
- **Auto-Reupload**: Triggers a fresh upload from source if the asset is corrupted.
- **Event Bus**: Publishes an `ImageRepaired` event for state synchronization.
