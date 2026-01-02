//! Tests for helper functions
//! Ported from compress-json/test/helpers-test.ts

use compress_json_rs::{trim_undefined, trim_undefined_recursively};
use serde_json::{Map, Value, json};

#[test]
fn test_trim_undefined_removes_null_fields() {
    let mut user: Map<String, Value> = serde_json::from_value(json!({
        "name": "Alice",
        "role": null,
        "after_undefined": null,
        "last": "value"
    }))
    .unwrap();

    trim_undefined(&mut user);

    // Null fields should be removed
    assert!(
        !user.contains_key("role"),
        "null field 'role' was not removed"
    );
    assert!(
        !user.contains_key("after_undefined"),
        "null field 'after_undefined' was not removed"
    );

    // Non-null fields should be preserved
    assert_eq!(user.get("name"), Some(&json!("Alice")));
    assert_eq!(user.get("last"), Some(&json!("value")));
}

#[test]
fn test_trim_undefined_preserves_non_null_values() {
    let mut data: Map<String, Value> = serde_json::from_value(json!({
        "str": "hello",
        "num": 42,
        "bool": true,
        "arr": [1, 2, 3],
        "obj": {"nested": "value"}
    }))
    .unwrap();

    let expected = data.clone();
    trim_undefined(&mut data);

    assert_eq!(data, expected, "Non-null values should be preserved");
}

#[test]
fn test_trim_undefined_empty_object() {
    let mut empty: Map<String, Value> = Map::new();
    trim_undefined(&mut empty);
    assert!(empty.is_empty(), "Empty object should remain empty");
}

#[test]
fn test_trim_undefined_all_null() {
    let mut all_null: Map<String, Value> = serde_json::from_value(json!({
        "a": null,
        "b": null,
        "c": null
    }))
    .unwrap();

    trim_undefined(&mut all_null);
    assert!(all_null.is_empty(), "All null fields should be removed");
}

#[test]
fn test_trim_undefined_recursively_removes_nested_nulls() {
    // Create nested structure with nulls
    let mut outer: Map<String, Value> = serde_json::from_value(json!({
        "name": "outer",
        "extra": null,
        "inner": {
            "name": "inner",
            "nested_null": null,
            "value": 42
        }
    }))
    .unwrap();

    trim_undefined_recursively(&mut outer);

    // Top-level null should be removed
    assert!(
        !outer.contains_key("extra"),
        "Top-level null 'extra' was not removed"
    );

    // Nested structure should exist
    let inner = outer.get("inner").expect("Inner object should exist");
    let inner_obj = inner.as_object().expect("Inner should be an object");

    // Nested null should be removed
    assert!(
        !inner_obj.contains_key("nested_null"),
        "Nested null field was not removed recursively"
    );

    // Non-null values should be preserved
    assert_eq!(inner_obj.get("name"), Some(&json!("inner")));
    assert_eq!(inner_obj.get("value"), Some(&json!(42)));
}

#[test]
fn test_trim_undefined_recursively_deeply_nested() {
    let mut data: Map<String, Value> = serde_json::from_value(json!({
        "level1": {
            "level2": {
                "level3": {
                    "value": "deep",
                    "null_field": null
                },
                "l2_null": null
            },
            "l1_null": null
        },
        "top_null": null,
        "preserved": "top"
    }))
    .unwrap();

    trim_undefined_recursively(&mut data);

    // Verify structure is correct and nulls are removed at all levels
    assert!(!data.contains_key("top_null"));
    assert_eq!(data.get("preserved"), Some(&json!("top")));

    let level1 = data.get("level1").unwrap().as_object().unwrap();
    assert!(!level1.contains_key("l1_null"));

    let level2 = level1.get("level2").unwrap().as_object().unwrap();
    assert!(!level2.contains_key("l2_null"));

    let level3 = level2.get("level3").unwrap().as_object().unwrap();
    assert!(!level3.contains_key("null_field"));
    assert_eq!(level3.get("value"), Some(&json!("deep")));
}

#[test]
fn test_trim_undefined_recursively_with_arrays() {
    // Arrays should not be modified (only objects are traversed)
    let mut data: Map<String, Value> = serde_json::from_value(json!({
        "arr": [1, null, 3],
        "nested": {
            "arr2": [null, "value"],
            "remove_me": null
        }
    }))
    .unwrap();

    trim_undefined_recursively(&mut data);

    // Array nulls are preserved (only object keys are trimmed)
    let arr = data.get("arr").unwrap();
    assert_eq!(arr, &json!([1, null, 3]));

    let nested = data.get("nested").unwrap().as_object().unwrap();
    assert!(!nested.contains_key("remove_me"));
    assert_eq!(nested.get("arr2"), Some(&json!([null, "value"])));
}

#[test]
fn test_trim_undefined_recursively_preserves_structure() {
    // Test that circular-reference-like structures (via same values) work
    let mut a: Map<String, Value> = serde_json::from_value(json!({
        "name": "a",
        "ref": "b_ref",
        "extra": null
    }))
    .unwrap();

    let mut b: Map<String, Value> = serde_json::from_value(json!({
        "name": "b",
        "a": {
            "name": "a",
            "ref": "b_ref",
            "extra": null
        }
    }))
    .unwrap();

    trim_undefined_recursively(&mut a);
    trim_undefined_recursively(&mut b);

    assert_eq!(a.get("name"), Some(&json!("a")));
    assert_eq!(a.get("ref"), Some(&json!("b_ref")));
    assert!(!a.contains_key("extra"));

    assert_eq!(b.get("name"), Some(&json!("b")));
    let b_a = b.get("a").unwrap().as_object().unwrap();
    assert_eq!(b_a.get("name"), Some(&json!("a")));
    assert!(!b_a.contains_key("extra"));
}
