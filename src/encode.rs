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
//! | Infinity | `N\|+` | `"N\|+"` (when preserved) |
//! | -Infinity | `N\|-` | `"N\|-"` (when preserved) |
//! | NaN | `N\|0` | `"N\|0"` (when preserved) |
//! | Escaped string | `s\|` | `"s\|n\|foo"` |
//! | Plain string | _(none)_ | `"hello"` |
//!
//! # String Escaping
//!
//! Strings that start with reserved prefixes (`b|`, `n|`, `N|`, `o|`, `a|`, `s|`)
//! are escaped with `s|` to prevent ambiguity during decoding.
//!
//! # Special Values (v3.4.0+)
//!
//! Special floating-point values have dedicated encodings when enabled via config:
//! - `Infinity` → `N|+` (when `preserve_infinite` is true)
//! - `-Infinity` → `N|-` (when `preserve_infinite` is true)
//! - `NaN` → `N|0` (when `preserve_nan` is true)
//!
//! When preservation is disabled (default), special values become `null` like `JSON.stringify`.
//! This ensures compatibility with JavaScript and Python implementations v3.4.0+.

use crate::number::s_to_int;

/// Encode a regular number to compressed string with 'n|' prefix.
///
/// This function is for regular (finite) numbers only. Special values
/// (Infinity, -Infinity, NaN) are handled separately in `memory.rs`
/// based on configuration settings.
///
/// # Arguments
///
/// * `num` - The f64 number to encode (should be finite)
///
/// # Returns
///
/// String in format `"n|<number>"`
///
/// # Example
///
/// ```ignore
/// assert_eq!(encode_num(42.5), "n|42.5");
/// assert_eq!(encode_num(-3.14), "n|-3.14");
/// assert_eq!(encode_num(0.0), "n|0");
/// ```
///
/// # Note
///
/// For special values (Infinity, NaN), the handling depends on config:
/// - `preserve_nan`/`preserve_infinite`: encoded as `N|0`, `N|+`, `N|-`
/// - Otherwise: converted to null (empty string)
pub fn encode_num(num: f64) -> String {
    format!("n|{num}")
}

/// Check if an encoded string represents a special value (Infinity/NaN).
///
/// # Arguments
///
/// * `s` - The encoded string to check
///
/// # Returns
///
/// `true` if the string starts with `N|` (special value prefix)
pub fn is_special_value(s: &str) -> bool {
    s.starts_with("N|")
}

/// Decode a special value string to f64.
///
/// # Arguments
///
/// * `s` - String starting with "N|" prefix
///
/// # Returns
///
/// - `f64::INFINITY` for "N|+"
/// - `f64::NEG_INFINITY` for "N|-"
/// - `f64::NAN` for "N|0"
///
/// # Panics
///
/// Panics if the string is not a valid special value encoding.
pub fn decode_special(s: &str) -> f64 {
    match s {
        "N|+" => f64::INFINITY,
        "N|-" => f64::NEG_INFINITY,
        "N|0" => f64::NAN,
        _ => panic!("Invalid special value encoding: {s}"),
    }
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
/// If the string starts with a reserved prefix (`b|`, `o|`, `n|`, `N|`, `a|`, `s|`),
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
/// assert_eq!(encode_str("N|+"), "s|N|+");     // Escaped (v3.2.0)
/// ```
pub fn encode_str(s: &str) -> String {
    // Check for reserved prefixes using starts_with (UTF-8 safe)
    // Note: N| added in v3.2.0 for special values
    if s.starts_with("b|")
        || s.starts_with("o|")
        || s.starts_with("n|")
        || s.starts_with("N|")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_special_values() {
        assert_eq!(decode_special("N|+"), f64::INFINITY);
        assert_eq!(decode_special("N|-"), f64::NEG_INFINITY);
        assert!(decode_special("N|0").is_nan());
    }

    #[test]
    fn test_encode_regular_numbers() {
        assert_eq!(encode_num(42.0), "n|42");
        assert_eq!(encode_num(-3.14), "n|-3.14");
        assert_eq!(encode_num(0.0), "n|0");
    }

    #[test]
    fn test_escape_special_prefix() {
        assert_eq!(encode_str("N|+"), "s|N|+");
        assert_eq!(encode_str("N|-"), "s|N|-");
        assert_eq!(encode_str("N|0"), "s|N|0");
    }
}
