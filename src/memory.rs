use std::collections::HashMap;
use serde_json::Value;
use crate::config::CONFIG;
use crate::debug::throw_unsupported_data;
use crate::encode::{encode_bool, encode_num, encode_str};
use crate::number::int_to_s;

/// Key type for compressed references
pub type Key = String;

/// In-memory structure holding store and caches for compression
pub struct Memory {
    store: Vec<String>,
    value_cache: HashMap<String, String>,
    schema_cache: HashMap<String, String>,
    key_count: usize,
}

/// Convert internal store to values array
pub fn mem_to_values(mem: &Memory) -> Vec<String> {
    mem.store.clone()
}

/// Create a new in-memory Memory instance
pub fn make_memory() -> Memory {
    Memory {
        store: Vec::new(),
        value_cache: HashMap::new(),
        schema_cache: HashMap::new(),
        key_count: 0,
    }
}

/// Get or insert a value in the store, returning its key
fn get_value_key(mem: &mut Memory, value: &str) -> String {
    if let Some(key) = mem.value_cache.get(value) {
        return key.clone();
    }
    let id = mem.key_count;
    let key = int_to_s(id);
    mem.key_count += 1;
    mem.store.push(value.to_string());
    mem.value_cache.insert(value.to_string(), key.clone());
    key
}

/// Get or insert a schema (object keys), returning its key
fn get_schema(mem: &mut Memory, keys: &[String]) -> String {
    let mut schema_keys = keys.to_vec();
    if CONFIG.sort_key {
        schema_keys.sort();
    }
    let schema = schema_keys.join(",");
    if let Some(key) = mem.schema_cache.get(&schema) {
        return key.clone();
    }
    // Represent schema as an array of strings
    let arr = Value::Array(
        schema_keys
            .iter()
            .map(|k| Value::String(k.clone()))
            .collect(),
    );
    let key_id = add_value(mem, &arr);
    mem.schema_cache.insert(schema, key_id.clone());
    key_id
}

/// Recursively add a JSON value to memory, returning its key
pub fn add_value(mem: &mut Memory, o: &Value) -> Key {
    match o {
        Value::Null => "".to_string(),
        Value::Bool(b) => get_value_key(mem, &encode_bool(*b)),
        Value::Number(n) => {
            // Convert number to f64
            let f = n.as_f64().unwrap_or_else(|| {
                // integer fallback
                n.as_i64()
                    .map(|i| i as f64)
                    .or_else(|| n.as_u64().map(|u| u as f64))
                    .unwrap_or(0.0)
            });
            if f.is_nan() {
                if CONFIG.error_on_nan {
                    throw_unsupported_data("[number NaN]");
                }
                return "".to_string();
            }
            if f.is_infinite() {
                if CONFIG.error_on_infinite {
                    throw_unsupported_data("[number Infinity]");
                }
                return "".to_string();
            }
            get_value_key(mem, &encode_num(f))
        }
        Value::String(s) => get_value_key(mem, &encode_str(s)),
        Value::Array(arr) => {
            let mut acc = String::from("a");
            for v in arr.iter() {
                let key = if v.is_null() {
                    "_".to_string()
                } else {
                    add_value(mem, v)
                };
                acc.push('|');
                acc.push_str(&key);
            }
            if acc == "a" {
                acc = "a|".to_string();
            }
            get_value_key(mem, &acc)
        }
        Value::Object(map) => {
            let keys: Vec<String> = map.keys().cloned().collect();
            if keys.is_empty() {
                return get_value_key(mem, "o|");
            }
            let key_id = get_schema(mem, &keys);
            let mut acc = String::from("o|");
            acc.push_str(&key_id);
            for key in keys.iter() {
                let v = &map[key];
                let val_key = add_value(mem, v);
                acc.push('|');
                acc.push_str(&val_key);
            }
            get_value_key(mem, &acc)
        }
    }
}