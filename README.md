# shortun — URL shortener

A sleek, minimal URL shortener written in Rust using Axum, SQLx and PostgreSQL. Designed for fast redirects, click stats, and easy deployment on Shuttle.

Features
- Shorten any valid HTTP/HTTPS URL
- Permanent redirects (301)
- Click stats per short URL (/\<id\>/stats)
- Built with async Rust, SQLx migrations, and Shuttle-ready configuration

Quickstart (local)
1. Install prerequisites:
   - Rust + Cargo (stable)
   - PostgreSQL
   - sqlx-cli (optional, for migrations): `cargo install sqlx-cli --no-default-features --features postgres`
2. Set DATABASE_URL:
   - export DATABASE_URL="postgres://user:password@localhost/dbname"
3. Run migrations:
   - `sqlx migrate run`
4. Run the app:
   - `cargo run`
5. By default the service expects requests at localhost:8000 (used in generated slugs when Host header is provided).

API
- POST /api/shorten
  - Body: JSON `{ "url": "https://example.com" }`
  - Response: 201 Created `{ "slug": "http://localhost:8000/abc123" }`

- GET /{id}
  - Redirects (301) to the original URL if found, otherwise 404.

- GET /{id}/stats
  - Returns JSON `{ "clicks": 42 }` or 404 if not found.

Examples
- Shorten
  - curl:
    - `curl -X POST -H "Content-Type: application/json" -d '{"url":"rust-lang.org"}' http://localhost:8000/api/shorten`
- Follow redirect
  - Open `http://localhost:8000/<id>` in a browser or:
    - `curl -I http://localhost:8000/<id>`
- Get stats
  - `curl http://localhost:8000/<id>/stats`

Deployment
- Project is Shuttle-ready. To deploy:
  - Install Shuttle CLI and follow Shuttle docs.
  - Push with required Postgres shared DB configuration; the app uses `shuttle_shared_db::Postgres`.

Notes
- URLs without a scheme will be normalized to `https://`.
- Only `http` and `https` schemes are allowed.
- Migrations are applied at startup (see `sqlx::migrate!()` in main).

Contributing
- Open an issue or PR. Keep changes small and documented.

License
- MIT