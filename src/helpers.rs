use serde_json::{Value, Map};

/// Remove keys with null values from a JSON object (shallow)
pub fn trim_undefined(object: &mut Map<String, Value>) {
    object.retain(|_, v| !v.is_null());
}

/// Recursively remove keys with null values in nested JSON objects
pub fn trim_undefined_recursively(object: &mut Map<String, Value>) {
    let mut tracks = Vec::new();
    fn recurse(obj: &mut Map<String, Value>, tracks: &mut Vec<*const Map<String, Value>>) {
        tracks.push(obj as *const _);
        let keys: Vec<String> = obj.keys().cloned().collect();
        for key in keys {
            if let Some(mut v) = obj.remove(&key) {
                if v.is_null() {
                    // skip insertion
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