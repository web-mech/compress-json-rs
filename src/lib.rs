// Module declarations
mod number;
mod encode;
mod boolean;
mod config;
mod debug;
mod memory;
mod helpers;
mod core;

// Re-export core functionality
pub use core::{compress, decompress, Compressed, decode};

// Expose lower-level APIs
pub use memory::{add_value, make_memory, mem_to_values, Key};
pub use helpers::{trim_undefined, trim_undefined_recursively};
pub use config::CONFIG;
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;

    #[test]
    fn test_payload_user_products_decompress() {
        // Load example payload.json from project root
        let text = fs::read_to_string("tests/payload.json").expect("Failed to read payload.json");
        let data: Value = serde_json::from_str(&text).expect("Invalid JSON in payload.json");
        // Extract compressed userProducts
        let compressed = &data["userProducts"];
        println!("userProducts compressed: {:?}", compressed);
        assert!(compressed.is_array(), "userProducts is not an array");
        let arr = compressed.as_array().unwrap();
        println!("compressed arr len={}, arr={:?}", arr.len(), arr);
        assert_eq!(arr.len(), 2, "compressed representation must have two elements");
        // First element: values array
        let values_json = &arr[0];
        let values: Vec<String> = values_json
            .as_array()
            .expect("values not array")
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        // Second element: root key
        let root = arr[1]
            .as_str()
            .expect("root key not a string")
            .to_string();
        println!("values.len={} root={}", values.len(), root);
        // Decompress
        let result = decompress((values, root));
        println!("decompressed result: {:?}", result);
        // Expect an array of product objects
        let products = result.as_array().expect("decompressed value is not an array");
        assert!(!products.is_empty(), "products array is empty");
        // Check that the first product's "id" field matches the compressed values entry
        let first = &products[0];
        assert!(first.is_object(), "first product is not an object");
        let first_id = first.get("id").and_then(Value::as_str).expect("first product has no id string");
        // The original compressed values array has the first product id at index 7
        let expected_id = data["userProducts"][0]
            .as_array().unwrap()[7]
            .as_str().unwrap();
        assert_eq!(first_id, expected_id, "decompressed id does not match");
    }
}
