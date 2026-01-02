//! Core compression and decompression functions.
//!
//! This module provides the main entry points for compressing and decompressing JSON values.
//!
//! # Compression
//!
//! The [`compress`] function takes a `serde_json::Value` and produces a [`Compressed`] tuple
//! containing the deduplicated value store and a root key.
//!
//! # Decompression
//!
//! The [`decompress`] function takes a [`Compressed`] tuple and reconstructs the original
//! JSON value.
//!
//! # Format
//!
//! Values are encoded with type prefixes:
//! - `b|T` / `b|F` - boolean true/false
//! - `n|<num>` - numeric value
//! - `N|+` - positive infinity (v3.2.0+)
//! - `N|-` - negative infinity (v3.2.0+)
//! - `N|0` - NaN (v3.2.0+)
//! - `s|<str>` - escaped string (for strings that look like encoded values)
//! - `a|<refs>` - array with pipe-separated element references
//! - `o|<schema>|<refs>` - object with schema reference and value references
//! - Plain string - unescaped string value
//! - Empty string or `_` - null value

use crate::encode::{
    decode_bool, decode_key, decode_num, decode_special, decode_str, is_special_value,
};
use crate::memory::{Key, add_value, make_memory, mem_to_values};
use serde_json::{Map, Number, Value};

/// Compressed representation: (values array, root key).
///
/// The first element is a vector of encoded strings representing all unique values.
/// The second element is a base-62 key pointing to the root value in the array.
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{compress, Compressed};
/// use serde_json::json;
///
/// let data = json!({"name": "Alice"});
/// let compressed: Compressed = compress(&data);
///
/// let (values, root) = compressed;
/// assert!(!values.is_empty());
/// assert!(!root.is_empty());
/// ```
pub type Compressed = (Vec<String>, Key);

/// Compress a JSON value into its compressed representation.
///
/// Takes any valid `serde_json::Value` and produces a compact, deduplicated
/// representation that can be serialized for storage or transmission.
///
/// # Arguments
///
/// * `o` - A reference to the JSON value to compress
///
/// # Returns
///
/// A [`Compressed`] tuple containing:
/// - `Vec<String>` - The deduplicated value store
/// - `String` - The base-62 key of the root value
///
/// # Example
///
/// ```rust
/// use compress_json_rs::compress;
/// use serde_json::json;
///
/// let data = json!({
///     "users": [
///         { "id": 1, "name": "Alice" },
///         { "id": 2, "name": "Bob" }
///     ]
/// });
///
/// let (values, root) = compress(&data);
///
/// // Values are deduplicated - the schema "id,name" appears once
/// println!("Compressed to {} values", values.len());
/// ```
///
/// # Handling Special Values (v3.2.0+)
///
/// Special floating-point values are now preserved:
/// - `Infinity` is encoded as `N|+`
/// - `-Infinity` is encoded as `N|-`
/// - `NaN` is encoded as `N|0`
///
/// Note: When `CONFIG.error_on_nan` or `CONFIG.error_on_infinite` is true,
/// these values will panic instead of being encoded.
///
/// Unicode strings are fully supported and strings that look like encoded
/// values (e.g., "n|123") are automatically escaped.
pub fn compress(o: &Value) -> Compressed {
    let mut mem = make_memory();
    let root = add_value(&mut mem, o);
    let values = mem_to_values(&mem);
    (values, root)
}

/// Decode an object from its encoded string representation.
fn decode_object(values: &Vec<String>, s: &str) -> Value {
    if s == "o|" {
        return Value::Object(Map::new());
    }
    let parts: Vec<&str> = s.split('|').collect();
    let key_id = parts[1];
    let keys_val = decode(values, key_id);
    let keys: Vec<String> = match keys_val {
        Value::String(ref k) => vec![k.clone()],
        Value::Array(arr) => arr
            .into_iter()
            .map(|v| match v {
                Value::String(s) => s,
                other => panic!("Invalid key type in decode_object: {other:?}"),
            })
            .collect(),
        other => panic!("Invalid keys in decode_object: {other:?}"),
    };
    let mut map = Map::new();
    for (i, part) in parts.iter().enumerate().skip(2) {
        let v = decode(values, part);
        let key = keys[i - 2].clone();
        map.insert(key, v);
    }
    Value::Object(map)
}

/// Decode an array from its encoded string representation.
fn decode_array(values: &Vec<String>, s: &str) -> Value {
    if s == "a|" {
        return Value::Array(Vec::new());
    }
    let parts: Vec<&str> = s.split('|').collect();
    let mut arr = Vec::with_capacity(parts.len() - 1);
    for part in parts.iter().skip(1) {
        let v = decode(values, part);
        arr.push(v);
    }
    Value::Array(arr)
}

/// Decode a single key into a JSON Value.
///
/// This is a lower-level function that decodes a single reference key
/// from the values array. It's used internally by [`decompress`] but
/// can also be used directly for custom decoding scenarios.
///
/// # Arguments
///
/// * `values` - The values array from a compressed representation
/// * `key` - A base-62 encoded key string
///
/// # Returns
///
/// The decoded `serde_json::Value`
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{compress, decode};
/// use serde_json::json;
///
/// let data = json!("hello");
/// let (values, root) = compress(&data);
///
/// let decoded = decode(&values, &root);
/// assert_eq!(decoded, json!("hello"));
/// ```
///
/// # Panics
///
/// Panics if the key references an invalid index or the encoded value is malformed.
pub fn decode(values: &Vec<String>, key: &str) -> Value {
    if key.is_empty() || key == "_" {
        return Value::Null;
    }
    let id = decode_key(key);
    let v_str = &values[id];
    // Determine value type by prefix and decode accordingly
    if v_str.starts_with("b|") {
        Value::Bool(decode_bool(v_str))
    } else if v_str.starts_with("o|") {
        decode_object(values, v_str)
    } else if is_special_value(v_str) {
        // Handle special values: N|+, N|-, N|0 (v3.2.0+)
        // Note: serde_json doesn't support Infinity/NaN directly,
        // so we return null for JSON compatibility
        let num = decode_special(v_str);
        if num.is_nan() || num.is_infinite() {
            // For JSON output, these become null
            // But the encoded form preserves the original value
            Value::Null
        } else {
            Value::Number(Number::from_f64(num).expect("Invalid number"))
        }
    } else if let Some(num_str) = v_str.strip_prefix("n|") {
        // Numeric: preserve integers when no decimal or exponent
        if !num_str.contains('.') && !num_str.contains('e') && !num_str.contains('E') {
            // try signed integer
            if let Ok(i) = num_str.parse::<i64>() {
                return Value::Number(Number::from(i));
            }
            // try unsigned integer
            if let Ok(u) = num_str.parse::<u64>() {
                return Value::Number(Number::from(u));
            }
        }
        // fallback to float
        let num = decode_num(v_str);
        Value::Number(Number::from_f64(num).expect("Invalid number"))
    } else if v_str.starts_with("a|") {
        decode_array(values, v_str)
    } else {
        // default to string
        Value::String(decode_str(v_str))
    }
}

/// Decompress a compressed representation back into JSON.
///
/// Takes a [`Compressed`] tuple produced by [`compress`] and reconstructs
/// the original JSON value.
///
/// # Arguments
///
/// * `c` - The compressed representation tuple
///
/// # Returns
///
/// The original `serde_json::Value`
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{compress, decompress};
/// use serde_json::json;
///
/// let original = json!({
///     "name": "Alice",
///     "scores": [95, 87, 92]
/// });
///
/// let compressed = compress(&original);
/// let restored = decompress(compressed);
///
/// assert_eq!(original, restored);
/// ```
///
/// # Round-trip Guarantee
///
/// For any valid JSON value, `decompress(compress(value))` will produce
/// an equivalent value. The only exceptions are:
/// - `NaN` and `Infinity` are encoded but become `null` in JSON output
///   (JSON doesn't support these values natively)
/// - Object key order may differ if `CONFIG.sort_key` was enabled
///
/// Note: The compressed form preserves `Infinity`, `-Infinity`, and `NaN`
/// with special encodings (`N|+`, `N|-`, `N|0`) for cross-platform
/// compatibility with JavaScript and Python implementations.
pub fn decompress(c: Compressed) -> Value {
    let (values, root) = c;
    decode(&values, &root)
}
