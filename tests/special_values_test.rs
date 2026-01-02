//! Tests for special floating-point values (v3.4.0 compatibility)
//!
//! Tests encoding/decoding of Infinity, -Infinity, and NaN
//! as specified in https://github.com/web-mech/compress-json-rs/issues/2
//!
//! v3.4.0 changes:
//! - Added `preserve_nan` and `preserve_infinite` config options
//! - Default behavior now converts NaN/Infinity to null (like JSON.stringify)
//! - Special encoding (N|+, N|-, N|0) only used when preserve options are enabled

use compress_json_rs::{compress, decompress};
use serde_json::json;

// Note: Since CONFIG is compile-time constant with preserve_* = false,
// special values become null by default. These tests verify that behavior.

#[test]
fn test_special_value_decoding() {
    use compress_json_rs::decode;
    
    // Simulate compressed data with special values (from another implementation
    // that has preserve_* enabled)
    let values = vec![
        "N|+".to_string(),  // index 0: Infinity
        "N|-".to_string(),  // index 1: -Infinity
        "N|0".to_string(),  // index 2: NaN
    ];
    
    // Decode each special value
    // Note: In JSON, these become null because JSON doesn't support Infinity/NaN
    let decoded_inf = decode(&values, "0");
    let decoded_neg_inf = decode(&values, "1");
    let decoded_nan = decode(&values, "2");
    
    // JSON representation is null (JSON spec limitation)
    assert!(decoded_inf.is_null());
    assert!(decoded_neg_inf.is_null());
    assert!(decoded_nan.is_null());
}

#[test]
fn test_regular_numbers_still_work() {
    let data = json!({
        "integer": 42,
        "float": 3.14159,
        "negative": -100,
        "zero": 0,
        "large": 9007199254740991_i64
    });
    
    let compressed = compress(&data);
    let decompressed = decompress(compressed);
    
    assert_eq!(data, decompressed);
}

#[test]
fn test_string_that_looks_like_special_value() {
    // Strings starting with N| should be escaped
    let data = json!({
        "fake_infinity": "N|+",
        "fake_neg_infinity": "N|-",
        "fake_nan": "N|0"
    });
    
    let compressed = compress(&data);
    let decompressed = decompress(compressed);
    
    assert_eq!(data, decompressed);
    assert_eq!(decompressed["fake_infinity"], "N|+");
    assert_eq!(decompressed["fake_neg_infinity"], "N|-");
    assert_eq!(decompressed["fake_nan"], "N|0");
}

#[test]
fn test_compressed_format_contains_special_encoding() {
    // Create values with the special encoding format directly
    // (simulating data from JS/Python with preserve_* enabled)
    let values = vec![
        "N|+".to_string(),
        "N|-".to_string(), 
        "N|0".to_string(),
        "n|42".to_string(),
        "a|0|1|2|3".to_string(),  // Array containing the values
    ];
    
    use compress_json_rs::decode;
    
    // Decode the array
    let decoded = decode(&values, "4");
    
    // The array should contain nulls (JSON representation) but the
    // encoded form preserves the original special values
    assert!(decoded.is_array());
    let arr = decoded.as_array().unwrap();
    assert_eq!(arr.len(), 4);
    
    // First three are special values (become null in JSON)
    assert!(arr[0].is_null());
    assert!(arr[1].is_null());
    assert!(arr[2].is_null());
    
    // Fourth is a regular number
    assert_eq!(arr[3], json!(42));
}

#[test]
fn test_decode_special_function() {
    use compress_json_rs::decode_special;
    
    // Test decode_special directly
    let inf = decode_special("N|+");
    assert!(inf.is_infinite() && inf.is_sign_positive());
    
    let neg_inf = decode_special("N|-");
    assert!(neg_inf.is_infinite() && neg_inf.is_sign_negative());
    
    let nan = decode_special("N|0");
    assert!(nan.is_nan());
}

#[test]
fn test_is_special_value() {
    use compress_json_rs::is_special_value;
    
    assert!(is_special_value("N|+"));
    assert!(is_special_value("N|-"));
    assert!(is_special_value("N|0"));
    
    assert!(!is_special_value("n|42"));
    assert!(!is_special_value("b|T"));
    assert!(!is_special_value("hello"));
}

#[test]
fn test_mixed_data_with_regular_values() {
    // Ensure special value handling doesn't break normal operations
    // Note: 88.0 becomes 88 (integer) since the library preserves integers
    let data = json!({
        "users": [
            {"id": 1, "name": "Alice", "score": 95.5},
            {"id": 2, "name": "Bob", "score": 88.1}  // Use non-whole number to test float
        ],
        "metadata": {
            "count": 2,
            "average": 91.75
        }
    });
    
    let compressed = compress(&data);
    let decompressed = decompress(compressed);
    
    assert_eq!(data, decompressed);
}

// ============================================================
// Tests for v3.4.0 default behavior (preserve_* = false)
// ============================================================

#[test]
fn test_default_config_nullifies_special_values() {
    // With default CONFIG (preserve_nan = false, preserve_infinite = false),
    // NaN and Infinity values would become null during compression.
    // Since serde_json::Number doesn't support NaN/Infinity, we can't create
    // them directly, but we can test that the decode handles null keys.
    
    use compress_json_rs::decode;
    
    // Empty key represents null
    let values = vec!["n|42".to_string()];
    let decoded_null = decode(&values, "");
    assert!(decoded_null.is_null());
    
    // "_" also represents null
    let decoded_null2 = decode(&values, "_");
    assert!(decoded_null2.is_null());
}

#[test]
fn test_cross_platform_decoding_compatibility() {
    // Data compressed with JS/Python v3.4.0+ with preserve_* = true
    // should be decodable in Rust (values become null in JSON)
    use compress_json_rs::decode;
    
    let values = vec![
        "N|+".to_string(),           // 0: Infinity
        "N|-".to_string(),           // 1: -Infinity  
        "N|0".to_string(),           // 2: NaN
        "n|42".to_string(),          // 3: regular number
        "hello".to_string(),         // 4: string
        "a|0|1|2|3|4".to_string(),   // 5: array of all values
    ];
    
    let decoded = decode(&values, "5");
    let arr = decoded.as_array().unwrap();
    
    // Special values become null in JSON output
    assert!(arr[0].is_null(), "Infinity should decode to null");
    assert!(arr[1].is_null(), "-Infinity should decode to null");
    assert!(arr[2].is_null(), "NaN should decode to null");
    assert_eq!(arr[3], json!(42), "Regular number should work");
    assert_eq!(arr[4], json!("hello"), "String should work");
}

#[test]
fn test_encode_num_for_regular_numbers() {
    use compress_json_rs::encode_num;
    
    // encode_num only handles regular (finite) numbers now
    assert_eq!(encode_num(42.0), "n|42");
    assert_eq!(encode_num(-3.14), "n|-3.14");
    assert_eq!(encode_num(0.0), "n|0");
    assert_eq!(encode_num(1e10), "n|10000000000");
}
