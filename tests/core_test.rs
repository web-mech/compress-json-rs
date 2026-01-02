//! Core compression/decompression tests
//! Ported from compress-json/test/core-test.ts

mod sample;

use compress_json_rs::{compress, decompress};
use serde_json::{Value, json};

/// Helper to test roundtrip compression/decompression
fn test_roundtrip(name: &str, data: Value) {
    test_roundtrip_with_expected(name, data.clone(), data);
}

/// Helper to test roundtrip with different expected output
/// (e.g., for NaN/Infinity which become null)
fn test_roundtrip_with_expected(name: &str, data: Value, expected: Value) {
    let compressed = compress(&data);
    let decompressed = decompress(compressed);

    assert_eq!(
        serde_json::to_string(&expected).unwrap(),
        serde_json::to_string(&decompressed).unwrap(),
        "Roundtrip failed for '{}'\nInput: {:?}\nExpected: {:?}\nGot: {:?}",
        name,
        data,
        expected,
        decompressed
    );
}

// ============================================================
// Sample data tests (matching TypeScript test order)
// ============================================================

#[test]
fn test_floating() {
    let data = sample::get_sample("floating");
    test_roundtrip("floating", data);
}

#[test]
fn test_rich() {
    let data = sample::get_sample("rich");
    test_roundtrip("rich", data);
}

#[test]
fn test_conflict() {
    let data = sample::get_sample("conflict");
    test_roundtrip("conflict", data);
}

#[test]
fn test_sparse() {
    let data = sample::get_sample("sparse");
    test_roundtrip("sparse", data);
}

#[test]
fn test_same_array() {
    let data = sample::get_sample("same_array");
    test_roundtrip("same_array", data);
}

#[test]
fn test_collection() {
    let data = sample::get_sample("collection");
    test_roundtrip("collection", data);
}

#[test]
fn test_exponential() {
    let data = sample::get_sample("exponential");
    test_roundtrip("exponential", data);
}

// ============================================================
// Issue #2 tests - empty/nested object handling
// ============================================================

#[test]
fn test_empty_object() {
    test_roundtrip("empty object", json!({}));
}

#[test]
fn test_object_with_one_key() {
    test_roundtrip("object with one key", json!({ "Name": "Triangle-01" }));
}

#[test]
fn test_nested_object_with_one_key() {
    test_roundtrip(
        "nested object with one key",
        json!({
            "Triangles": { "Name": "Triangle-01" }
        }),
    );
}

#[test]
fn test_nested_object_with_same_key() {
    test_roundtrip(
        "nested object with same key",
        json!({
            "Triangles": { "Triangles": { "Name": "Triangle-01" } }
        }),
    );
}

#[test]
fn test_nested_object_and_array_with_same_key() {
    test_roundtrip(
        "nested object and array with same key",
        json!({
            "Triangles": { "Triangles": [{ "Name": "Triangle-01" }] }
        }),
    );
}

#[test]
fn test_single_key_object_using_existing_value_as_key() {
    test_roundtrip(
        "Single-key object using existing value as key",
        json!({
            "Name": "Start",
            "Triangles": {
                "Name": "Triangle-01"
            }
        }),
    );
}

#[test]
fn test_array_using_existing_values_used_by_object_key() {
    test_roundtrip(
        "Array using existing values used by object key",
        json!({
            "obj": {
                "id": 1,
                "name": "arr"
            },
            "str": "id,name"
        }),
    );
}

// ============================================================
// Object.prototype conflict tests
// ============================================================

#[test]
fn test_object_prototype_conflicts() {
    test_roundtrip(
        "Handle name conflict with Object.prototype",
        json!({
            "toString": 1,
            "valueOf": 2,
            "hasOwnProperty": 3,
            "constructor": 4,
            "isPrototypeOf": 5,
            "propertyIsEnumerable": 6
        }),
    );
}

// ============================================================
// Issue #5 tests - string appears as both key and value
// ============================================================

#[test]
fn test_string_as_key_and_value() {
    test_roundtrip(
        "A string appears as both key and value",
        json!({
            "any-1": {
                "key-and-value": "any-3"
            },
            "any-2": "key-and-value"
        }),
    );
}

// ============================================================
// Empty array and null handling tests
// ============================================================

#[test]
fn test_empty_array() {
    test_roundtrip("empty array", json!([]));
}

#[test]
fn test_array_with_null_element() {
    let data = json!([null]);
    let compressed = compress(&data);
    let decompressed = decompress(compressed);

    // Verify the null is preserved (not undefined)
    let arr = decompressed.as_array().expect("Expected array");
    assert_eq!(arr.len(), 1);
    assert!(arr[0].is_null(), "Expected null, got {:?}", arr[0]);
}

#[test]
fn test_array_with_multiple_null_elements() {
    test_roundtrip("array with multiple null elements", json!([null, null]));
}

// ============================================================
// Issue #21 tests - invalid numbers (NaN, Infinity)
// ============================================================

#[test]
fn test_invalid_numbers_become_null() {
    // In JSON output, NaN and Infinity become null because JSON doesn't support them.
    // However, the compressed form preserves them with special encodings:
    // - Infinity -> N|+
    // - -Infinity -> N|-
    // - NaN -> N|0
    // This test verifies basic null handling; see special_values_test.rs for v3.2.0 tests
    test_roundtrip(
        "array with nulls for invalid numbers",
        json!([null, null, null]),
    );
}

// ============================================================
// Additional edge case tests
// ============================================================

#[test]
fn test_deeply_nested_structure() {
    test_roundtrip(
        "deeply nested structure",
        json!({
            "a": {
                "b": {
                    "c": {
                        "d": {
                            "e": "deep value"
                        }
                    }
                }
            }
        }),
    );
}

#[test]
fn test_mixed_array_types() {
    test_roundtrip(
        "mixed array types",
        json!([
            1,
            "string",
            true,
            false,
            null,
            { "key": "value" },
            [1, 2, 3]
        ]),
    );
}

#[test]
fn test_unicode_strings() {
    test_roundtrip(
        "unicode strings",
        json!({
            "emoji": "🎉🚀💻",
            "chinese": "中文测试",
            "japanese": "日本語テスト",
            "arabic": "اختبار العربية"
        }),
    );
}

#[test]
fn test_special_characters_in_strings() {
    test_roundtrip(
        "special characters in strings",
        json!({
            "newline": "line1\nline2",
            "tab": "col1\tcol2",
            "quote": "say \"hello\"",
            "backslash": "path\\to\\file"
        }),
    );
}

#[test]
fn test_duplicate_values_compression() {
    // Test that duplicate values are efficiently compressed
    let data = json!({
        "items": [
            { "type": "fruit", "name": "apple", "color": "red" },
            { "type": "fruit", "name": "banana", "color": "yellow" },
            { "type": "fruit", "name": "cherry", "color": "red" },
            { "type": "vegetable", "name": "carrot", "color": "orange" },
            { "type": "fruit", "name": "grape", "color": "purple" }
        ]
    });

    let compressed = compress(&data);
    let decompressed = decompress(compressed);

    assert_eq!(
        serde_json::to_string(&data).unwrap(),
        serde_json::to_string(&decompressed).unwrap()
    );
}

#[test]
fn test_large_integers() {
    test_roundtrip(
        "large integers",
        json!({
            "small": 1,
            "medium": 1000000,
            "large": 9007199254740991_i64, // MAX_SAFE_INTEGER
            "negative_large": -9007199254740991_i64
        }),
    );
}

#[test]
fn test_boolean_values() {
    test_roundtrip(
        "boolean values",
        json!({
            "true_val": true,
            "false_val": false,
            "arr": [true, false, true]
        }),
    );
}

#[test]
fn test_null_values_in_objects() {
    test_roundtrip(
        "null values in objects",
        json!({
            "present": "value",
            "absent": null,
            "nested": {
                "inner_present": 42,
                "inner_null": null
            }
        }),
    );
}

#[test]
fn test_escaped_prefix_strings() {
    // Strings that look like encoded values should be escaped
    test_roundtrip(
        "escaped prefix strings",
        json!({
            "looks_like_string": "s|actual content",
            "looks_like_number": "n|not a number",
            "looks_like_object": "o|not an object",
            "looks_like_array": "a|not an array",
            "looks_like_bool": "b|not a bool"
        }),
    );
}

#[test]
fn test_repeated_keys_in_arrays_of_objects() {
    // Schema should be reused for objects with same keys
    test_roundtrip(
        "repeated keys in arrays of objects",
        json!([
            { "id": 1, "name": "first" },
            { "id": 2, "name": "second" },
            { "id": 3, "name": "third" }
        ]),
    );
}

#[test]
fn test_complex_real_world_like_data() {
    test_roundtrip(
        "complex real-world-like data",
        json!({
            "users": [
                {
                    "id": "user-001",
                    "profile": {
                        "firstName": "John",
                        "lastName": "Doe",
                        "email": "john@example.com"
                    },
                    "settings": {
                        "theme": "dark",
                        "notifications": true,
                        "language": "en"
                    },
                    "active": true
                },
                {
                    "id": "user-002",
                    "profile": {
                        "firstName": "Jane",
                        "lastName": "Smith",
                        "email": "jane@example.com"
                    },
                    "settings": {
                        "theme": "light",
                        "notifications": false,
                        "language": "en"
                    },
                    "active": true
                }
            ],
            "metadata": {
                "version": "1.0.0",
                "timestamp": "2024-01-15T10:30:00Z"
            }
        }),
    );
}

// ============================================================
// Primitives at root level
// ============================================================

#[test]
fn test_string_root() {
    test_roundtrip("string at root", json!("hello world"));
}

#[test]
fn test_number_root() {
    test_roundtrip("number at root", json!(42.5));
}

#[test]
fn test_boolean_true_root() {
    test_roundtrip("true at root", json!(true));
}

#[test]
fn test_boolean_false_root() {
    test_roundtrip("false at root", json!(false));
}

#[test]
fn test_null_root() {
    test_roundtrip("null at root", Value::Null);
}

#[test]
fn test_array_root() {
    test_roundtrip("array at root", json!([1, 2, 3]));
}
