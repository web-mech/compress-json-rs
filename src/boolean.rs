//! Boolean encoding utilities.
//!
//! This module provides simple boolean to string conversion functions.
//! These are lower-level utilities; the main encoding uses the `encode`
//! module's `encode_bool` and `decode_bool` functions instead.
//!
//! # Encoding
//!
//! | Value | Encoded |
//! |-------|---------|
//! | `true` | `"T"` |
//! | `false` | `"F"` |

/// Convert boolean to compressed string.
///
/// # Arguments
///
/// * `b` - Boolean value
///
/// # Returns
///
/// `"T"` for true, `"F"` for false
#[allow(dead_code)]
pub fn bool_to_s(b: bool) -> String {
    if b { "T".to_string() } else { "F".to_string() }
}

/// Convert compressed string to boolean.
///
/// # Arguments
///
/// * `s` - String "T" or "F"
///
/// # Returns
///
/// `true` for "T", `false` for "F" or empty string
#[allow(dead_code)]
pub fn s_to_bool(s: &str) -> bool {
    match s {
        "T" => true,
        "F" => false,
        _ => !s.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_roundtrip() {
        assert_eq!(s_to_bool(&bool_to_s(true)), true);
        assert_eq!(s_to_bool(&bool_to_s(false)), false);
    }
}
