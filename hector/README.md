# hector

Serve your application's source code at a `/hector` endpoint.

hector embeds your source files into your binary **at compile time** and serves them as a downloadable `.tar.gz` archive via [axum](https://github.com/tokio-rs/axum) — no filesystem access required at runtime.

See the [repository](https://github.com/neiam/hector) for full documentation.

## Usage

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

`GET /hector` returns a `sources.tar.gz` containing your source as it was at compile time.

## License

MIT
