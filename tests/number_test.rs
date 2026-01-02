//! Tests for number encoding/decoding functionality
//! Ported from compress-json/test/number-test.ts

use compress_json_rs::{compress, decompress};
use serde_json::json;

/// Test that a number roundtrips correctly through compress/decompress
fn test_number_roundtrip(x: f64) {
    let value = json!(x);
    let compressed = compress(&value);
    let decompressed = decompress(compressed);

    // Compare as f64 values to handle JSON representation
    let result = decompressed.as_f64().expect("Expected a number");
    assert_eq!(x, result, "Number roundtrip failed for {}", x);
}

#[test]
fn test_large_integer() {
    // Test large integer: 1234567890
    test_number_roundtrip(1234567890.0);
}

#[test]
fn test_large_integer_2() {
    // Test large integer: 987654321
    test_number_roundtrip(987654321.0);
}

#[test]
fn test_decimal_number() {
    // Test decimal number: 1234.4321
    test_number_roundtrip(1234.4321);
}

#[test]
fn test_negative_number() {
    // Test negative number: -5
    test_number_roundtrip(-5.0);
}

#[test]
fn test_precision_bug() {
    // Test for precision bug reported in https://github.com/beenotung/compress-json/issues/3
    test_number_roundtrip(0.12371134020618557);
}

#[test]
fn test_exponential_large() {
    // Test exponential number suggested in https://github.com/beenotung/compress-json/issues/9
    test_number_roundtrip(1.23456789123789e22);
}

#[test]
fn test_exponential_small_e9() {
    // Test for exponential bug reported in https://github.com/beenotung/compress-json/issues/12
    test_number_roundtrip(1.2e-9);
}

#[test]
fn test_exponential_small_e10() {
    // Test for exponential bug reported in https://github.com/beenotung/compress-json/issues/12
    test_number_roundtrip(1.2e-10);
}

#[test]
fn test_exponential_e21() {
    // Test for exponential bug reported in https://github.com/beenotung/compress-json/issues/22
    test_number_roundtrip(1e21);
}

#[test]
fn test_exponential_e_minus_13() {
    // Test for exponential bug reported in https://github.com/beenotung/compress-json/issues/22
    test_number_roundtrip(2e-13);
}

#[test]
fn test_overflow_division() {
    // Test for overflow bug reported in https://github.com/beenotung/compress-json/issues/16
    test_number_roundtrip(1.0 / 12.0);
}

#[test]
fn test_integer_preservation() {
    // Ensure integers are preserved as integers
    let value = json!(42);
    let compressed = compress(&value);
    let decompressed = decompress(compressed);

    // Should be an integer, not a float
    assert!(
        decompressed.is_i64() || decompressed.is_u64(),
        "Expected integer, got {:?}",
        decompressed
    );
    assert_eq!(value, decompressed);
}

#[test]
fn test_negative_integer() {
    let value = json!(-42);
    let compressed = compress(&value);
    let decompressed = decompress(compressed);
    assert_eq!(value, decompressed);
}

#[test]
fn test_zero() {
    let value = json!(0);
    let compressed = compress(&value);
    let decompressed = decompress(compressed);
    assert_eq!(value, decompressed);
}

#[test]
fn test_max_safe_integer() {
    // JavaScript's MAX_SAFE_INTEGER equivalent
    let value = json!(9007199254740991_i64);
    let compressed = compress(&value);
    let decompressed = decompress(compressed);
    assert_eq!(value, decompressed);
}

#[test]
fn test_min_safe_integer() {
    // JavaScript's MIN_SAFE_INTEGER equivalent
    let value = json!(-9007199254740991_i64);
    let compressed = compress(&value);
    let decompressed = decompress(compressed);
    assert_eq!(value, decompressed);
}

#[test]
fn test_all_numbers_comprehensive() {
    // Comprehensive test matching TypeScript's test array
    let numbers = vec![
        1234567890.0,
        987654321.0,
        1234.4321,
        -5.0,
        0.12371134020618557,
        1.23456789123789e22,
        1.2e-9,
        1.2e-10,
        1e21,
        2e-13,
        1.0 / 12.0,
    ];

    for x in numbers {
        test_number_roundtrip(x);
    }
}
