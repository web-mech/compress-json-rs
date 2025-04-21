use crate::number::s_to_int;

/// Encode a number to compressed string with 'n|' prefix (unused)
#[allow(dead_code)]
pub fn encode_num(num: f64) -> String {
    format!("n|{}", num.to_string())
}

/// Decode a compressed number string to f64
pub fn decode_num(s: &str) -> f64 {
    let s2 = s.strip_prefix("n|").unwrap_or(s);
    s2.parse::<f64>().expect("invalid number")
}

/// Decode a key string (base-N) to an index
pub fn decode_key(key: &str) -> usize {
    s_to_int(key)
}

/// Encode a boolean to compressed string with 'b|' prefix
pub fn encode_bool(b: bool) -> String {
    if b {
        "b|T".to_string()
    } else {
        "b|F".to_string()
    }
}

/// Decode a compressed boolean string to bool
pub fn decode_bool(s: &str) -> bool {
    match s {
        "b|T" => true,
        "b|F" => false,
        _ => !s.is_empty(),
    }
}

/// Encode a generic string, escaping reserved prefixes with 's|' if needed
pub fn encode_str(s: &str) -> String {
    if s.len() >= 2 {
        let prefix = &s[0..2];
        match prefix {
            "b|" | "o|" | "n|" | "a|" | "s|" => return format!("s|{}", s),
            _ => {}
        }
    }
    s.to_string()
}

/// Decode a compressed string, unescaping 's|' prefix if present
pub fn decode_str(s: &str) -> String {
    if s.starts_with("s|") {
        s[2..].to_string()
    } else {
        s.to_string()
    }
}