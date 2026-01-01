//! Memory management for compression state.
//!
//! This module provides the [`Memory`] struct and related functions for managing
//! the compression state. It handles value deduplication through caching and
//! schema sharing for objects with identical keys.
//!
//! # Architecture
//!
//! The memory system consists of:
//! - **Store**: A vector of encoded string values
//! - **Value Cache**: HashMap for deduplicating identical values
//! - **Schema Cache**: HashMap for deduplicating object schemas (key lists)
//!
//! # Deduplication
//!
//! When a value is added:
//! 1. It's first checked against the value cache
//! 2. If found, the existing key is returned (no new storage)
//! 3. If not found, a new key is generated and the value is stored
//!
//! This ensures that identical values (like repeated strings or numbers)
//! are only stored once.
//!
//! # Example
//!
//! ```rust
//! use compress_json_rs::{make_memory, add_value, mem_to_values};
//! use serde_json::json;
//!
//! let mut mem = make_memory();
//!
//! // Adding the same value twice returns the same key
//! let key1 = add_value(&mut mem, &json!("repeated"));
//! let key2 = add_value(&mut mem, &json!("repeated"));
//! assert_eq!(key1, key2);
//!
//! // The value is only stored once
//! let values = mem_to_values(&mem);
//! assert_eq!(values.len(), 1);
//! ```

use std::collections::HashMap;
use serde_json::Value;
use crate::config::CONFIG;
use crate::debug::throw_unsupported_data;
use crate::encode::{encode_bool, encode_num, encode_str};
use crate::number::int_to_s;

/// Key type for compressed references.
///
/// Keys are base-62 encoded strings that reference positions in the values array.
/// The base-62 encoding uses characters `0-9`, `A-Z`, and `a-z`.
///
/// # Examples
///
/// - `"0"` - First value (index 0)
/// - `"A"` - Eleventh value (index 10)
/// - `"10"` - Sixty-third value (index 62)
pub type Key = String;

/// In-memory structure holding store and caches for compression.
///
/// This struct maintains the state needed during compression. It uses
/// internal caching to deduplicate values and object schemas.
///
/// # Fields (Internal)
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `store` | `Vec<String>` | Encoded string values |
/// | `value_cache` | `HashMap` | Maps values to keys |
/// | `schema_cache` | `HashMap` | Maps schemas to keys |
/// | `key_count` | `usize` | Key counter |
///
/// # Usage
///
/// Create with [`make_memory`], add values with [`add_value`], and extract
/// the final values array with [`mem_to_values`].
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{Memory, make_memory, add_value, mem_to_values};
/// use serde_json::json;
///
/// // Create memory store
/// let mut mem: Memory = make_memory();
///
/// // Add values (duplicates are deduplicated)
/// let k1 = add_value(&mut mem, &json!("hello"));
/// let k2 = add_value(&mut mem, &json!("hello"));
/// assert_eq!(k1, k2);
///
/// // Extract values
/// let values = mem_to_values(&mem);
/// assert_eq!(values.len(), 1);
/// ```
pub struct Memory {
    /// The actual stored values (encoded strings)
    pub(crate) store: Vec<String>,
    /// Cache mapping encoded values to their keys
    pub(crate) value_cache: HashMap<String, String>,
    /// Cache mapping object schemas to their keys
    pub(crate) schema_cache: HashMap<String, String>,
    /// Counter for generating sequential keys
    pub(crate) key_count: usize,
}

/// Convert internal store to values array.
///
/// Extracts the values vector from a `Memory` instance. This is typically
/// called after all values have been added to get the final compressed output.
///
/// # Arguments
///
/// * `mem` - Reference to the Memory instance
///
/// # Returns
///
/// A clone of the internal values vector
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{make_memory, add_value, mem_to_values};
/// use serde_json::json;
///
/// let mut mem = make_memory();
/// add_value(&mut mem, &json!({"key": "value"}));
/// let values = mem_to_values(&mem);
/// assert!(!values.is_empty());
/// ```
pub fn mem_to_values(mem: &Memory) -> Vec<String> {
    mem.store.clone()
}

/// Create a new in-memory Memory instance.
///
/// Initializes an empty `Memory` struct ready to accept values.
///
/// # Returns
///
/// A new, empty Memory instance
///
/// # Example
///
/// ```rust
/// use compress_json_rs::make_memory;
///
/// let mem = make_memory();
/// // Ready to use with add_value()
/// ```
pub fn make_memory() -> Memory {
    Memory {
        store: Vec::new(),
        value_cache: HashMap::new(),
        schema_cache: HashMap::new(),
        key_count: 0,
    }
}

/// Get or insert a value in the store, returning its key.
///
/// This is the core deduplication function. It checks if the encoded value
/// already exists in the cache, returning the existing key if so. Otherwise,
/// it generates a new key, stores the value, and caches the mapping.
fn get_value_key(mem: &mut Memory, value: &str) -> String {
    if let Some(key) = mem.value_cache.get(value) {
        return key.clone();
    }
    let id = mem.key_count;
    let key = int_to_s(id);
    mem.key_count += 1;
    mem.store.push(value.to_string());
    mem.value_cache.insert(value.to_string(), key.clone());
    key
}

/// Get or insert a schema (object keys), returning its key.
///
/// Schemas are stored as arrays of key strings. Objects with identical
/// keys share the same schema, reducing storage for arrays of similar objects.
fn get_schema(mem: &mut Memory, keys: &[String]) -> String {
    let mut schema_keys = keys.to_vec();
    if CONFIG.sort_key {
        schema_keys.sort();
    }
    let schema = schema_keys.join(",");
    if let Some(key) = mem.schema_cache.get(&schema) {
        return key.clone();
    }
    // Represent schema as an array of strings
    let arr = Value::Array(
        schema_keys
            .iter()
            .map(|k| Value::String(k.clone()))
            .collect(),
    );
    let key_id = add_value(mem, &arr);
    mem.schema_cache.insert(schema, key_id.clone());
    key_id
}

/// Recursively add a JSON value to memory, returning its key.
///
/// This function handles all JSON value types and recursively processes
/// nested arrays and objects. Values are deduplicated through the internal
/// cache.
///
/// # Arguments
///
/// * `mem` - Mutable reference to the Memory instance
/// * `o` - Reference to the JSON value to add
///
/// # Returns
///
/// A base-62 encoded key string referencing the stored value
///
/// # Value Encoding
///
/// | Type | Encoding | Example |
/// |------|----------|---------|
/// | Null | Empty string | `""` |
/// | Bool | `b\|T` or `b\|F` | `"b\|T"` |
/// | Number | `n\|<value>` | `"n\|42.5"` |
/// | String | Plain or `s\|<escaped>` | `"hello"` or `"s\|n\|123"` |
/// | Array | `a\|<refs>` | `"a\|0\|1\|2"` |
/// | Object | `o\|<schema>\|<refs>` | `"o\|0\|1\|2"` |
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{make_memory, add_value, mem_to_values, decode};
/// use serde_json::json;
///
/// let mut mem = make_memory();
///
/// // Add a complex value
/// let key = add_value(&mut mem, &json!({
///     "name": "Alice",
///     "scores": [95, 87, 92]
/// }));
///
/// // The key can be used to decode back
/// let values = mem_to_values(&mem);
/// let decoded = decode(&values, &key);
/// assert_eq!(decoded["name"], "Alice");
/// ```
///
/// # Special Cases
///
/// - **NaN**: Returns empty key (null) unless `CONFIG.error_on_nan` is true
/// - **Infinity**: Returns empty key (null) unless `CONFIG.error_on_infinite` is true
/// - **Null in arrays**: Encoded as `_` to distinguish from empty references
pub fn add_value(mem: &mut Memory, o: &Value) -> Key {
    match o {
        Value::Null => "".to_string(),
        Value::Bool(b) => get_value_key(mem, &encode_bool(*b)),
        Value::Number(n) => {
            // Convert number to f64
            let f = n.as_f64().unwrap_or_else(|| {
                // integer fallback
                n.as_i64()
                    .map(|i| i as f64)
                    .or_else(|| n.as_u64().map(|u| u as f64))
                    .unwrap_or(0.0)
            });
            if f.is_nan() {
                if CONFIG.error_on_nan {
                    throw_unsupported_data("[number NaN]");
                }
                return "".to_string();
            }
            if f.is_infinite() {
                if CONFIG.error_on_infinite {
                    throw_unsupported_data("[number Infinity]");
                }
                return "".to_string();
            }
            get_value_key(mem, &encode_num(f))
        }
        Value::String(s) => get_value_key(mem, &encode_str(s)),
        Value::Array(arr) => {
            let mut acc = String::from("a");
            for v in arr.iter() {
                let key = if v.is_null() {
                    "_".to_string()
                } else {
                    add_value(mem, v)
                };
                acc.push('|');
                acc.push_str(&key);
            }
            if acc == "a" {
                acc = "a|".to_string();
            }
            get_value_key(mem, &acc)
        }
        Value::Object(map) => {
            let keys: Vec<String> = map.keys().cloned().collect();
            if keys.is_empty() {
                return get_value_key(mem, "o|");
            }
            let key_id = get_schema(mem, &keys);
            let mut acc = String::from("o|");
            acc.push_str(&key_id);
            for key in keys.iter() {
                let v = &map[key];
                let val_key = add_value(mem, v);
                acc.push('|');
                acc.push_str(&val_key);
            }
            get_value_key(mem, &acc)
        }
    }
}
