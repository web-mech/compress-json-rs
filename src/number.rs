//! Base-62 number encoding for compact key representation.
//!
//! This module provides functions to convert between integers and base-62
//! strings. Base-62 uses characters `0-9`, `A-Z`, and `a-z` (62 total),
//! allowing compact representation of array indices.
//!
//! # Character Set
//!
//! ```text
//! Index:  0  1  2 ... 9 10 11 ... 35 36 37 ... 61
//! Char:  '0' '1' '2'   '9' 'A' 'B'   'Z' 'a' 'b'   'z'
//! ```
//!
//! # Examples
//!
//! | Index | Base-62 Key |
//! |-------|-------------|
//! | 0 | `"0"` |
//! | 9 | `"9"` |
//! | 10 | `"A"` |
//! | 35 | `"Z"` |
//! | 36 | `"a"` |
//! | 61 | `"z"` |
//! | 62 | `"10"` |
//! | 124 | `"20"` |
//! | 3844 | `"100"` |

/// Character set for base-62 encoding: 0-9, A-Z, a-z
const ITO_S: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Base of the encoding (62 characters)
const N: usize = ITO_S.len();

/// Convert base-62 string to integer index.
///
/// Decodes a base-62 encoded string back to its numeric value.
///
/// # Arguments
///
/// * `s` - Base-62 encoded string
///
/// # Returns
///
/// The decoded integer value
///
/// # Panics
///
/// Panics if the string contains characters not in the base-62 character set.
///
/// # Example
///
/// ```ignore
/// assert_eq!(s_to_int("0"), 0);
/// assert_eq!(s_to_int("A"), 10);
/// assert_eq!(s_to_int("10"), 62);
/// ```
pub fn s_to_int(s: &str) -> usize {
    let mut acc = 0;
    let mut pow = 1;
    for c in s.chars().rev() {
        let idx = ITO_S.find(c).expect("invalid character in s_to_int");
        acc += idx * pow;
        pow *= N;
    }
    acc
}

/// Convert integer to base-62 string.
///
/// Encodes a numeric index as a compact base-62 string.
///
/// # Arguments
///
/// * `value` - The integer to encode
///
/// # Returns
///
/// Base-62 encoded string
///
/// # Example
///
/// ```ignore
/// assert_eq!(int_to_s(0), "0");
/// assert_eq!(int_to_s(10), "A");
/// assert_eq!(int_to_s(62), "10");
/// ```
#[allow(dead_code)]
pub fn int_to_s(value: usize) -> String {
    if value == 0 {
        return ITO_S.chars().next().unwrap().to_string();
    }
    let mut val = value;
    let mut acc = Vec::new();
    while val != 0 {
        let i = val % N;
        let c = ITO_S.chars().nth(i).unwrap();
        acc.push(c);
        val /= N;
    }
    acc.iter().rev().collect()
}

/// Reverse a string.
#[allow(dead_code)]
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base62_roundtrip() {
        for i in 0..1000 {
            let encoded = int_to_s(i);
            let decoded = s_to_int(&encoded);
            assert_eq!(i, decoded, "Failed for {}", i);
        }
    }

    #[test]
    fn test_known_values() {
        assert_eq!(int_to_s(0), "0");
        assert_eq!(int_to_s(9), "9");
        assert_eq!(int_to_s(10), "A");
        assert_eq!(int_to_s(35), "Z");
        assert_eq!(int_to_s(36), "a");
        assert_eq!(int_to_s(61), "z");
        assert_eq!(int_to_s(62), "10");
    }
}
