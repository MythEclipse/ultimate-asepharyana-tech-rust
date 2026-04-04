# RustExpress API

Backend service berbasis Axum untuk scraping, image proxy/cache, observability, dan endpoint API inti.

## Stack

- Rust + Axum
- SeaORM (MySQL)
- Redis (deadpool-redis)
- Prometheus metrics
- Utoipa + Swagger UI

## Quick Start

```bash
cargo run
```

Server bind ke `0.0.0.0:${PORT}` dengan default port `4091`.

## Required Environment Variables

```env
DATABASE_URL=mysql://asephs:hunterz@localhost:3306/sosmed
JWT_SECRET=change-me
REDIS_URL=redis://localhost:6379
```

Optional yang sering dipakai:

```env
RUST_LOG=info
EXTERNAL_BROWSERLESS_WS=
MINIO_ENDPOINT=
MINIO_BUCKET_NAME=
MINIO_ACCESS_KEY=
MINIO_SECRET_KEY=
```

## Useful Endpoints

- `GET /docs` - Swagger UI
- `GET /api-docs/openapi.json` - OpenAPI JSON
- `GET /metrics` - Prometheus metrics
- `GET /api/anime2/*`
- `GET /api/komik/*`
- `POST /api/proxy/image-cache`
- `POST /api/proxy/image-cache/audit`

## Development References

- [docs/scraping.md](docs/scraping.md)
- [docs/proxy_cdn.md](docs/proxy_cdn.md)
- [docs/observability.md](docs/observability.md)
- [docs/api_reference.md](docs/api_reference.md)
- [docs/development.md](docs/development.md)

## Notes

- Database akan dicek/dibuat saat startup jika `DATABASE_URL` bertipe MySQL.
- Service ini juga menginisialisasi browser pool dan scheduler saat boot.

## License

MIT
