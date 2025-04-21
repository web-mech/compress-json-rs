# compress-json-rs

[![crates.io](https://img.shields.io/crates/v/compress-json-rs)](https://crates.io/crates/compress-json-rs) [![docs.rs](https://docs.rs/compress-json-rs/badge.svg)](https://docs.rs/compress-json-rs)

AI-driven Rust port of the JavaScript [compress-json](https://github.com/beenotung/compress-json) library by Beenotung.
Store JSON data in a space-efficient compressed form with lossless round-trip compression and decompression.

## Features

- Supports all JSON types: objects, arrays, strings, numbers, booleans, and null
- Deduplicates repeated values and keys for optimal size
- Encodes numbers in compact base-62 format
- Zero-copy round-trip using `serde_json::Value`
- Fast in-memory compression; no disk or network dependencies

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
compress-json-rs = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Quick Start

```rust
use compress_json_rs::{compress, decompress};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Original JSON data
    let data: Value = json!({
        "user": "Alice",
        "active": true,
        "roles": ["admin", "user"]
    });

    // Compress into (values, root_key)
    let compressed = compress(&data);

    // Serialize compressed form for storage or transmission
    let serialized = serde_json::to_string(&compressed)?;

    // ...later, deserialize and decompress
    let (values, root): (Vec<String>, String) = serde_json::from_str(&serialized)?;
    let restored = decompress((values, root));

    assert_eq!(restored, data);
    println!("Round-trip successful: {}", restored);
    Ok(())
}
```

## API Reference

```rust
/// Compressed representation: (values array, root key)
pub type Compressed = (Vec<String>, String);

/// Compress a JSON value into its compressed form
pub fn compress(o: &serde_json::Value) -> Compressed;

/// Decompress a compressed form back into JSON
pub fn decompress(c: Compressed) -> serde_json::Value;
```

See [`docs.rs`](https://docs.rs/compress-json-rs) for full lower-level API details.

## License

Licensed under the BSD-2-Clause license. See [LICENSE](LICENSE) for details.
