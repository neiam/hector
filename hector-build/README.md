# hector-build

Build-script helper for [hector](https://crates.io/crates/hector).

Walks your source tree at compile time and produces a gzipped tar archive embedded into your binary via `hector::sources!()`.

## Usage

```toml
# Cargo.toml
[build-dependencies]
hector-build = "0.1"
```

```rust
// build.rs
fn main() {
    hector_build::collect_sources(".");
}
```

See the [hector crate](https://crates.io/crates/hector) for the full example.

## License

MIT
