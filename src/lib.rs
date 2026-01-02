//! # compress-json-rs
//!
//! [![Crates.io](https://img.shields.io/crates/v/compress-json-rs.svg)](https://crates.io/crates/compress-json-rs)
//! [![Documentation](https://docs.rs/compress-json-rs/badge.svg)](https://docs.rs/compress-json-rs)
//! [![License](https://img.shields.io/crates/l/compress-json-rs.svg)](https://github.com/web-mech/compress-json-rs/blob/main/LICENSE)
//!
//! A space-efficient JSON compression library with **lossless round-trip** compression and decompression.
//!
//! This crate compresses JSON data by deduplicating values and keys, storing them in a compact
//! format with base-62 encoded references. It's particularly effective for JSON with repetitive
//! structures like API responses, configuration files, and data collections.
//!
//! # Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | **Full JSON Support** | Objects, arrays, strings, numbers, booleans, and null |
//! | **Value Deduplication** | Repeated values stored once with reference keys |
//! | **Schema Deduplication** | Objects with identical keys share schemas |
//! | **Compact Keys** | Base-62 encoding for minimal key size |
//! | **UTF-8 Safe** | Full Unicode support for all string values |
//! | **Zero Dependencies** | Only requires `serde_json` |
//!
//! # Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! compress-json-rs = "0.1"
//! serde_json = "1.0"
//! ```
//!
//! Basic usage:
//!
//! ```rust
//! use compress_json_rs::{compress, decompress};
//! use serde_json::json;
//!
//! // Original JSON data
//! let data = json!({
//!     "user": "Alice",
//!     "active": true,
//!     "roles": ["admin", "user"]
//! });
//!
//! // Compress into (values, root_key)
//! let compressed = compress(&data);
//!
//! // Decompress back to original
//! let restored = decompress(compressed);
//!
//! assert_eq!(data, restored);
//! ```
//!
//! # API Overview
//!
//! ## Core Functions
//!
//! | Function | Description |
//! |----------|-------------|
//! | [`compress`] | Compress a JSON value into [`Compressed`] format |
//! | [`decompress`] | Restore original JSON from [`Compressed`] format |
//! | [`decode`] | Decode a single key from the values array |
//!
//! ## Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Compressed`] | Tuple type `(Vec<String>, String)` for compressed data |
//! | [`Key`] | String type alias for base-62 encoded references |
//! | [`CONFIG`] | Global configuration constants |
//!
//! ## Helper Functions
//!
//! | Function | Description |
//! |----------|-------------|
//! | [`trim_undefined`] | Remove null values from object (shallow) |
//! | [`trim_undefined_recursively`] | Remove null values from nested objects |
//!
//! ## Low-Level API
//!
//! | Function | Description |
//! |----------|-------------|
//! | [`make_memory`] | Create a new compression memory store |
//! | [`add_value`] | Add a value to memory, get its key |
//! | [`mem_to_values`] | Extract values array from memory |
//!
//! # Compression Format
//!
//! The [`Compressed`] type is a tuple `(Vec<String>, String)`:
//! - First element: Deduplicated value store
//! - Second element: Base-62 key pointing to root value
//!
//! ## Value Encoding
//!
//! Values are encoded with type prefixes:
//!
//! | Prefix | Type | Example Encoded | Original Value |
//! |--------|------|-----------------|----------------|
//! | `b\|T` | Boolean | `b\|T` | `true` |
//! | `b\|F` | Boolean | `b\|F` | `false` |
//! | `n\|` | Number | `n\|42.5` | `42.5` |
//! | `N\|+` | Infinity | `N\|+` | `Infinity` |
//! | `N\|-` | -Infinity | `N\|-` | `-Infinity` |
//! | `N\|0` | NaN | `N\|0` | `NaN` |
//! | `s\|` | Escaped string | `s\|n\|foo` | `"n\|foo"` |
//! | `a\|` | Array | `a\|0\|1\|2` | `[val0, val1, val2]` |
//! | `o\|` | Object | `o\|0\|1\|2` | `{schema0: val1, ...}` |
//! | _(none)_ | String | `hello` | `"hello"` |
//! | `""` / `_` | Null | | `null` |
//!
//! ## Base-62 Keys
//!
//! Keys use characters `0-9`, `A-Z`, `a-z` for compact representation:
//!
//! ```text
//! Index:  0  1  2 ... 9  10 11 ... 35 36 37 ... 61 62  63
//! Key:   "0" "1" "2"   "9" "A" "B"   "Z" "a" "b"   "z" "10" "11"
//! ```
//!
//! # Examples
//!
//! ## Serialize for Storage
//!
//! ```rust
//! use compress_json_rs::{compress, decompress, Compressed};
//! use serde_json::json;
//!
//! let data = json!({"items": [1, 2, 3]});
//!
//! // Compress and serialize to JSON string
//! let compressed = compress(&data);
//! let json_str = serde_json::to_string(&compressed).unwrap();
//!
//! // Store json_str to file/database/network...
//!
//! // Later: deserialize and decompress
//! let loaded: Compressed = serde_json::from_str(&json_str).unwrap();
//! let restored = decompress(loaded);
//!
//! assert_eq!(data, restored);
//! ```
//!
//! ## Arrays of Similar Objects
//!
//! Compression is most effective for repetitive data:
//!
//! ```rust
//! use compress_json_rs::{compress, decompress};
//! use serde_json::json;
//!
//! // Data with repeated schema and values
//! let users = json!([
//!     { "id": 1, "name": "Alice", "role": "user" },
//!     { "id": 2, "name": "Bob", "role": "user" },
//!     { "id": 3, "name": "Charlie", "role": "admin" },
//! ]);
//!
//! let (values, root) = compress(&users);
//!
//! // Schema ["id", "name", "role"] stored once
//! // Value "user" stored once, referenced twice
//! println!("Compressed to {} unique values", values.len());
//!
//! let restored = decompress((values, root));
//! assert_eq!(users, restored);
//! ```
//!
//! ## Clean Data Before Compression
//!
//! ```rust
//! use compress_json_rs::{compress, trim_undefined_recursively};
//! use serde_json::{json, Map, Value};
//!
//! let mut data: Map<String, Value> = serde_json::from_value(json!({
//!     "name": "Alice",
//!     "middleName": null,  // Will be removed
//!     "profile": {
//!         "bio": "Developer",
//!         "website": null   // Will be removed
//!     }
//! })).unwrap();
//!
//! // Remove all null values recursively
//! trim_undefined_recursively(&mut data);
//!
//! // Now compress the cleaned data
//! let compressed = compress(&Value::Object(data));
//! ```
//!
//! ## Low-Level API Usage
//!
//! For custom compression workflows:
//!
//! ```rust
//! use compress_json_rs::{make_memory, add_value, mem_to_values, decode};
//! use serde_json::json;
//!
//! let mut mem = make_memory();
//!
//! // Add values - duplicates return same key
//! let key1 = add_value(&mut mem, &json!("repeated"));
//! let key2 = add_value(&mut mem, &json!("repeated"));
//! assert_eq!(key1, key2); // Same key!
//!
//! // Add more values
//! let key3 = add_value(&mut mem, &json!(42));
//! let key4 = add_value(&mut mem, &json!({"nested": "object"}));
//!
//! // Extract final values
//! let values = mem_to_values(&mem);
//!
//! // Decode any key
//! assert_eq!(decode(&values, &key1), json!("repeated"));
//! assert_eq!(decode(&values, &key3), json!(42));
//! ```
//!
//! # Performance
//!
//! ## Best Use Cases
//!
//! - **API responses** with arrays of similar objects
//! - **Configuration files** with repeated values
//! - **Data exports** with consistent schemas
//! - **Cache storage** where size matters
//!
//! ## Compression Ratios
//!
//! | Data Type | Typical Ratio |
//! |-----------|---------------|
//! | Arrays of similar objects | 30-50% of original |
//! | Highly repetitive data | 20-40% of original |
//! | Mixed data | 50-70% of original |
//! | Unique values only | ~100% (no benefit) |
//!
//! # Special Values (v3.4.0+)
//!
//! This crate supports special floating-point values for cross-platform compatibility.
//! Handling depends on configuration options:
//!
//! | Config Option | Default | Effect |
//! |---------------|---------|--------|
//! | `preserve_nan` | `false` | When `true`, NaN encoded as `N\|0` |
//! | `preserve_infinite` | `false` | When `true`, ±Infinity encoded as `N\|+`/`N\|-` |
//! | `error_on_nan` | `false` | When `true` (and preserve=false), panic on NaN |
//! | `error_on_infinite` | `false` | When `true` (and preserve=false), panic on Infinity |
//!
//! By default (preserve options = false), special values become `null` like `JSON.stringify`.
//!
//! When preserved, the encoding is:
//! - `Infinity` → `N|+`
//! - `-Infinity` → `N|-`
//! - `NaN` → `N|0`
//!
//! Note: JSON doesn't natively support these values, so they become `null` when
//! decompressed to `serde_json::Value`. The compressed representation preserves
//! them for compatibility with JavaScript and Python implementations that have
//! `preserve_*` enabled.
//!
//! # Compatibility
//!
//! This crate is a Rust port of the JavaScript [compress-json](https://github.com/beenotung/compress-json)
//! library (v3.4.0+). Compressed data is compatible between implementations, allowing cross-platform
//! data exchange.
//!
//! # License
//!
//! Licensed under BSD-2-Clause. See [LICENSE](https://github.com/web-mech/compress-json-rs/blob/main/LICENSE).

#![doc(html_root_url = "https://docs.rs/compress-json-rs/0.3.1")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

// Module declarations
mod boolean;
mod config;
mod core;
mod debug;
mod encode;
mod helpers;
mod memory;
mod number;

// Re-export core functionality
pub use core::{Compressed, compress, decode, decompress};

// Expose lower-level APIs
pub use config::{CONFIG, Config};
pub use helpers::{trim_undefined, trim_undefined_recursively};
pub use memory::{Key, Memory, add_value, make_memory, mem_to_values};

// Expose encoding functions for special values (v3.2.0+)
pub use encode::{decode_num, decode_special, encode_num, is_special_value};
