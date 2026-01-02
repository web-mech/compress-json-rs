//! Helper utility functions for JSON processing.
//!
//! This module provides utility functions for cleaning up JSON data before
//! or after compression, particularly for handling null/undefined values.
//!
//! # Functions
//!
//! - [`trim_undefined`] - Remove null values from an object (shallow)
//! - [`trim_undefined_recursively`] - Remove null values from nested objects
//!
//! # Use Cases
//!
//! These functions are useful when:
//! - Working with data that may contain optional/null fields
//! - Reducing payload size by removing empty values
//! - Normalizing data before compression
//!
//! # Note on Naming
//!
//! The functions are named `trim_undefined` for compatibility with the
//! JavaScript version, where `undefined` and `null` have different semantics.
//! In Rust/JSON, these functions operate on `null` values.

use serde_json::{Map, Value};

/// Remove keys with null values from a JSON object (shallow).
///
/// This function modifies the object in place, removing any key-value pairs
/// where the value is `null`. It only operates on the top level of the object;
/// nested objects are not affected.
///
/// # Arguments
///
/// * `object` - Mutable reference to a JSON object map
///
/// # Example
///
/// ```rust
/// use compress_json_rs::trim_undefined;
/// use serde_json::{json, Map, Value};
///
/// let mut data: Map<String, Value> = serde_json::from_value(json!({
///     "name": "Alice",
///     "email": null,
///     "age": 30,
///     "phone": null
/// })).unwrap();
///
/// trim_undefined(&mut data);
///
/// // Only non-null values remain
/// assert_eq!(data.len(), 2);
/// assert!(data.contains_key("name"));
/// assert!(data.contains_key("age"));
/// assert!(!data.contains_key("email"));
/// assert!(!data.contains_key("phone"));
/// ```
///
/// # Note
///
/// Arrays within the object are not modified. If an object contains an array
/// with null elements, those null elements will remain.
pub fn trim_undefined(object: &mut Map<String, Value>) {
    object.retain(|_, v| !v.is_null());
}

/// Recursively remove keys with null values in nested JSON objects.
///
/// This function traverses the entire object tree, removing any key-value
/// pairs where the value is `null` at all levels of nesting.
///
/// # Arguments
///
/// * `object` - Mutable reference to a JSON object map
///
/// # Example
///
/// ```rust
/// use compress_json_rs::trim_undefined_recursively;
/// use serde_json::{json, Map, Value};
///
/// let mut data: Map<String, Value> = serde_json::from_value(json!({
///     "user": {
///         "name": "Bob",
///         "middleName": null,
///         "address": {
///             "street": "123 Main St",
///             "apt": null
///         }
///     },
///     "metadata": null
/// })).unwrap();
///
/// trim_undefined_recursively(&mut data);
///
/// // Check structure
/// assert!(!data.contains_key("metadata"));
///
/// let user = data.get("user").unwrap().as_object().unwrap();
/// assert!(!user.contains_key("middleName"));
///
/// let address = user.get("address").unwrap().as_object().unwrap();
/// assert!(!address.contains_key("apt"));
/// assert!(address.contains_key("street"));
/// ```
///
/// # Behavior
///
/// - Only object values are traversed recursively
/// - Arrays are not traversed (null elements in arrays remain)
/// - The function handles cyclic references safely (via pointer tracking)
pub fn trim_undefined_recursively(object: &mut Map<String, Value>) {
    let mut tracks = Vec::new();
    fn recurse(obj: &mut Map<String, Value>, tracks: &mut Vec<*const Map<String, Value>>) {
        tracks.push(obj as *const _);
        let keys: Vec<String> = obj.keys().cloned().collect();
        for key in keys {
            if let Some(v) = obj.remove(&key) {
                if v.is_null() {
                    // skip insertion - effectively removes the key
                } else {
                    match v {
                        Value::Object(mut m) => {
                            let ptr = &m as *const _;
                            if !tracks.contains(&ptr) {
                                recurse(&mut m, tracks);
                            }
                            obj.insert(key, Value::Object(m));
                        }
                        other => {
                            obj.insert(key, other);
                        }
                    }
                }
            }
        }
    }
    recurse(object, &mut tracks);
}
