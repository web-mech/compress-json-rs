/// Panic indicating an unsupported data type encountered
pub fn throw_unknown_data_type() -> ! {
    panic!("unsupported data type");
}

/// Panic indicating unsupported data value with a description
pub fn throw_unsupported_data(name: &str) -> ! {
    panic!("unsupported data type: {}", name);
}