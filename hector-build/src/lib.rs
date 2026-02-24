use flate2::{Compression, write::GzEncoder};
use std::env;
use std::fs;
use std::path::Path;
use tar::Builder;
use walkdir::WalkDir;

/// Directories to skip entirely when walking the source tree.
const SKIP_DIRS: &[&str] = &["target", ".git", ".hg", ".svn", "node_modules"];

/// Walk `source_dir` and create a gzipped tar archive at `$OUT_DIR/hector_sources.tar.gz`.
/// The [`hector::sources!`] macro embeds it directly via `include_bytes!`.
///
/// Pass `"."` (or `env!("CARGO_MANIFEST_DIR")`) to capture `Cargo.toml`, `build.rs`,
/// and `src/` together so the tarball is self-contained and buildable.
///
/// ```rust,ignore
/// fn main() {
///     hector_build::collect_sources(".");
/// }
/// ```
pub fn collect_sources(source_dir: impl AsRef<Path>) {
    let source_dir = source_dir.as_ref();
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let tarball_path = Path::new(&out_dir).join("hector_sources.tar.gz");

    let mut files: Vec<(String, std::path::PathBuf)> = Vec::new();

    for entry in WalkDir::new(source_dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Prune well-known directories we never want to embed.
            if e.file_type().is_dir() {
                if let Some(name) = e.file_name().to_str() {
                    return !SKIP_DIRS.contains(&name);
                }
            }
            true
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };

        let include = match path.extension().and_then(|e| e.to_str()) {
            Some("rs") | Some("toml") => true,
            _ => {
                // No extension: Cargo.lock, or entirely uppercase names (README, LICENSE, …)
                file_name == "Cargo.lock"
                    || (!file_name.contains('.') && file_name.chars().all(|c| c.is_ascii_uppercase()))
            }
        };

        if !include {
            continue;
        }

        println!("cargo:rerun-if-changed={}", path.display());

        let rel = path.to_string_lossy().into_owned();
        files.push((rel, path.to_path_buf()));
    }

    println!("cargo:rerun-if-changed={}", source_dir.display());

    let tar_gz_bytes = {
        let buf = Vec::new();
        let enc = GzEncoder::new(buf, Compression::best());
        let mut ar = Builder::new(enc);

        for (rel, path) in &files {
            let data = match fs::read(path) {
                Ok(d) => d,
                Err(_) => continue,
            };
            let mut header = tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            ar.append_data(&mut header, rel, data.as_slice())
                .expect("failed to add file to tar");
        }

        let enc = ar.into_inner().expect("failed to finalize tar");
        enc.finish().expect("failed to finalize gzip")
    };

    fs::write(&tarball_path, &tar_gz_bytes).expect("failed to write hector_sources.tar.gz");
}
