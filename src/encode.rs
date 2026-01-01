//! Encoding and decoding functions for compressed values.
//!
//! This module provides internal functions for encoding JSON values into
//! their compressed string representations and decoding them back.
//!
//! # Encoding Format
//!
//! | Type | Prefix | Example |
//! |------|--------|---------|
//! | Boolean true | `b\|T` | `"b\|T"` |
//! | Boolean false | `b\|F` | `"b\|F"` |
//! | Number | `n\|` | `"n\|42.5"` |
//! | Escaped string | `s\|` | `"s\|n\|foo"` |
//! | Plain string | _(none)_ | `"hello"` |
//!
//! # String Escaping
//!
//! Strings that start with reserved prefixes (`b|`, `n|`, `o|`, `a|`, `s|`)
//! are escaped with `s|` to prevent ambiguity during decoding.

use crate::number::s_to_int;

/// Encode a number to compressed string with 'n|' prefix.
///
/// # Arguments
///
/// * `num` - The f64 number to encode
///
/// # Returns
///
/// String in format `"n|<number>"`
///
/// # Example
///
/// ```ignore
/// let encoded = encode_num(42.5);
/// assert_eq!(encoded, "n|42.5");
/// ```
#[allow(dead_code)]
pub fn encode_num(num: f64) -> String {
    format!("n|{num}")
}

/// Decode a compressed number string to f64.
///
/// # Arguments
///
/// * `s` - String starting with "n|" prefix
///
/// # Returns
///
/// The decoded f64 value
///
/// # Panics
///
/// Panics if the string after the prefix is not a valid number.
pub fn decode_num(s: &str) -> f64 {
    let s2 = s.strip_prefix("n|").unwrap_or(s);
    s2.parse::<f64>().expect("invalid number")
}

/// Decode a key string (base-62) to an index.
///
/// Converts a base-62 encoded key back to its numeric index
/// in the values array.
///
/// # Arguments
///
/// * `key` - Base-62 encoded key string
///
/// # Returns
///
/// The numeric index as usize
pub fn decode_key(key: &str) -> usize {
    s_to_int(key)
}

/// Encode a boolean to compressed string with 'b|' prefix.
///
/// # Arguments
///
/// * `b` - Boolean value to encode
///
/// # Returns
///
/// `"b|T"` for true, `"b|F"` for false
pub fn encode_bool(b: bool) -> String {
    if b {
        "b|T".to_string()
    } else {
        "b|F".to_string()
    }
}

/// Decode a compressed boolean string to bool.
///
/// # Arguments
///
/// * `s` - String "b|T" or "b|F"
///
/// # Returns
///
/// `true` for "b|T", `false` for "b|F" or empty string
pub fn decode_bool(s: &str) -> bool {
    match s {
        "b|T" => true,
        "b|F" => false,
        _ => !s.is_empty(),
    }
}

/// Encode a string, escaping reserved prefixes with 's|' if needed.
///
/// If the string starts with a reserved prefix (`b|`, `o|`, `n|`, `a|`, `s|`),
/// it's escaped by prepending `s|` to prevent decoding ambiguity.
///
/// # Arguments
///
/// * `s` - The string to encode
///
/// # Returns
///
/// The original string, or escaped with `s|` prefix if needed
///
/// # Example
///
/// ```ignore
/// assert_eq!(encode_str("hello"), "hello");
/// assert_eq!(encode_str("n|123"), "s|n|123"); // Escaped
/// ```
pub fn encode_str(s: &str) -> String {
    // Check for reserved prefixes using starts_with (UTF-8 safe)
    if s.starts_with("b|") 
        || s.starts_with("o|") 
        || s.starts_with("n|") 
        || s.starts_with("a|") 
        || s.starts_with("s|") 
    {
        return format!("s|{s}");
    }
    s.to_string()
}

/// Decode a compressed string, unescaping 's|' prefix if present.
///
/// # Arguments
///
/// * `s` - The encoded string
///
/// # Returns
///
/// The original string with `s|` prefix removed if present
pub fn decode_str(s: &str) -> String {
    // Use strip_prefix for safe UTF-8 handling
    if let Some(stripped) = s.strip_prefix("s|") {
        stripped.to_string()
    } else {
        s.to_string()
    }
}
