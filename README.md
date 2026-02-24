# hector

Serve your application's source code at a `/hector` endpoint.

hector is a pair of crates for [axum](https://github.com/tokio-rs/axum) that embeds your source files into your binary **at compile time** and serves them as a downloadable `.tar.gz` archive — no filesystem access required at runtime.

Useful for auditing exactly what code is running in a deployed service.

---

## Quick start

```toml
# Cargo.toml
[dependencies]
hector = "0.1"

[build-dependencies]
hector-build = "0.1"
```

```rust
// build.rs
fn main() {
    hector_build::collect_sources(".");
}
```

```rust
// src/main.rs
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(hector::router(hector::sources!()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

`GET /hector` now returns a `sources.tar.gz` download containing your source as it was **at compile time**.

---

## What gets included

| Pattern | Example |
|---|---|
| `*.rs` | `src/main.rs`, `src/lib.rs` |
| `*.toml` | `Cargo.toml` |
| `Cargo.lock` | `Cargo.lock` |
| All-uppercase filenames | `README`, `LICENSE`, `CHANGELOG` |

`target/`, `.git/`, and other non-source directories are skipped automatically.

---

## How it works

**`hector-build`** runs in your build script. It walks the source tree, packs matching files into a gzipped tar archive, and writes it to Cargo's `OUT_DIR`.

**`hector::sources!()`** expands to `include_bytes!(...)` pointing at that archive, embedding the bytes directly into your binary.

**`hector::router()`** mounts a `GET /hector` handler that serves those bytes with `Content-Type: application/gzip` and `Content-Disposition: attachment; filename="sources.tar.gz"`.

---

## Crates

| Crate | Role | Add as |
|---|---|---|
| [`hector`](https://crates.io/crates/hector) | axum router + `sources!()` macro | `[dependencies]` |
| [`hector-build`](https://crates.io/crates/hector-build) | build-script helper | `[build-dependencies]` |

---

## License

MIT
