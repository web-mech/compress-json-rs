/// Convert boolean to compressed string
#[allow(dead_code)]
pub fn bool_to_s(b: bool) -> String {
    if b {
        "T".to_string()
    } else {
        "F".to_string()
    }
}

/// Convert compressed string to boolean
#[allow(dead_code)]
pub fn s_to_bool(s: &str) -> bool {
    match s {
        "T" => true,
        "F" => false,
        _ => !s.is_empty(),
    }
}