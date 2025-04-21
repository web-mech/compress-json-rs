use std::collections::HashMap;
use serde_json::{Value, Map, Number};
use crate::encode::{decode_bool, decode_key, decode_num, decode_str};
use crate::memory::{make_memory, mem_to_values, add_value, Key};

/// Compressed representation: (values array, root key)
pub type Compressed = (Vec<String>, Key);

/// Compress a JSON object into its compressed representation
pub fn compress(o: &Value) -> Compressed {
    let mut mem = make_memory();
    let root = add_value(&mut mem, o);
    let values = mem_to_values(&mem);
    (values, root)
}

fn decode_object(values: &Vec<String>, s: &str) -> Value {
    if s == "o|" {
        return Value::Object(Map::new());
    }
    let parts: Vec<&str> = s.split('|').collect();
    let key_id = parts[1];
    let mut keys_val = decode(values, key_id);
    let mut keys: Vec<String> = match keys_val {
        Value::String(ref k) => vec![k.clone()],
        Value::Array(arr) => arr.into_iter().map(|v| match v {
            Value::String(s) => s,
            other => panic!("Invalid key type in decode_object: {:?}", other),
        }).collect(),
        other => panic!("Invalid keys in decode_object: {:?}", other),
    };
    let mut map = Map::new();
    for (i, part) in parts.iter().enumerate().skip(2) {
        let v = decode(values, part);
        let key = keys[i - 2].clone();
        map.insert(key, v);
    }
    Value::Object(map)
}

fn decode_array(values: &Vec<String>, s: &str) -> Value {
    if s == "a|" {
        return Value::Array(Vec::new());
    }
    let parts: Vec<&str> = s.split('|').collect();
    let mut arr = Vec::with_capacity(parts.len() - 1);
    for part in parts.iter().skip(1) {
        let v = decode(values, part);
        arr.push(v);
    }
    Value::Array(arr)
}

/// Decode a single key into a JSON Value
pub fn decode(values: &Vec<String>, key: &str) -> Value {
    if key.is_empty() || key == "_" {
        return Value::Null;
    }
    let id = decode_key(key);
    let v_str = &values[id];
    let prefix = &v_str[0..2];
    match prefix {
        "b|" => Value::Bool(decode_bool(v_str)),
        "o|" => decode_object(values, v_str),
        "n|" => {
            let num = decode_num(v_str);
            Value::Number(Number::from_f64(num).expect("Invalid number"))
        }
        "a|" => decode_array(values, v_str),
        _ => {
            // default to string
            let s = decode_str(v_str);
            Value::String(s)
        }
    }
}

/// Decompress a compressed representation back into JSON
pub fn decompress(c: Compressed) -> Value {
    let (values, root) = c;
    decode(&values, &root)
}