//! Test sample data generation
//! Ported from compress-json/test/sample.ts

use serde_json::{Value, json};

/// Sample names for generating test data
const SAMPLE_NAMES: &[&str] = &[
    "Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Henry", "Ivy", "Jack",
];

/// Generate sample test data matching the TypeScript version
pub fn sample() -> Value {
    let long_str = "A very very long string, that is repeated";
    let long_num = 9876543210.123456_f64;

    // Create sparse array (array with gaps)
    let mut sparse: Vec<Value> = vec![Value::Null; 11];
    sparse[10] = json!(1);

    // Generate collection of user-like objects
    let collection: Vec<Value> = (0..10)
        .map(|i| {
            json!({
                "user_id": i + 1,
                "name": SAMPLE_NAMES[i % SAMPLE_NAMES.len()],
                "region": "HK",
                "role": "user",
                "more": "fields"
            })
        })
        .collect();

    json!({
        "floating": [
            0.12371134020618557,
            0.032989690721649485,
            0.18144329896907216,
            2.1651785714285716
        ],
        "rich": {
            "int": 42,
            "float": 12.34,
            "str": "Alice",
            "longStr": long_str,
            "longNum": long_num,
            "bool": true,
            "bool2": false,
            "arr": [42, long_str],
            "arr2": [42, long_str],
            "obj": {
                "id": 123,
                "name": "Alice",
                "role": ["Admin", "User", "Guest"],
                "longStr": long_str,
                "longNum": long_num
            },
            "escape": ["s|str", "n|123", "o|1", "a|1", "b|T", "b|F", "...s|..."]
        },
        "conflict": {
            "str": "1",
            "num": 1
        },
        "sparse": sparse,
        "same_array": {
            "arr_1": [1, 2, 3, 4, 5],
            "arr_2": [1, 2, 3, 4, 5]
        },
        "collection": collection,
        "exponential": [
            1.23456789123789e22,
            1.23456789123789e-22,
            -1.23456789123789e22,
            -1.23456789123789e-22
        ]
    })
}

/// Get a specific sample section by name
pub fn get_sample(name: &str) -> Value {
    let all = sample();
    all.get(name).cloned().unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_structure() {
        let data = sample();
        assert!(data.is_object());

        // Check all expected keys exist
        let obj = data.as_object().unwrap();
        assert!(obj.contains_key("floating"));
        assert!(obj.contains_key("rich"));
        assert!(obj.contains_key("conflict"));
        assert!(obj.contains_key("sparse"));
        assert!(obj.contains_key("same_array"));
        assert!(obj.contains_key("collection"));
        assert!(obj.contains_key("exponential"));
    }

    #[test]
    fn test_sparse_array() {
        let sparse = get_sample("sparse");
        let arr = sparse.as_array().unwrap();
        assert_eq!(arr.len(), 11);
        assert!(arr[0].is_null());
        assert_eq!(arr[10], json!(1));
    }

    #[test]
    fn test_collection_size() {
        let collection = get_sample("collection");
        let arr = collection.as_array().unwrap();
        assert_eq!(arr.len(), 10);
    }
}
