//! Debug and error handling utilities.
//!
//! This module provides panic functions for handling unsupported data
//! types and values during compression.
//!
//! # Error Handling
//!
//! These functions are called when the compression encounters invalid
//! data (like NaN or Infinity) and the configuration is set to error
//! rather than silently converting to null.

/// Panic indicating an unsupported data type encountered.
///
/// This function is called when an unknown or unsupported JSON value
/// type is encountered during compression.
///
/// # Panics
///
/// Always panics with message "unsupported data type"
#[allow(dead_code)]
pub fn throw_unknown_data_type() -> ! {
    panic!("unsupported data type");
}

/// Panic indicating unsupported data value with a description.
///
/// This function is called when a specific unsupported value is
/// encountered, such as NaN or Infinity when configured to error.
///
/// # Arguments
///
/// * `name` - Description of the unsupported value
///
/// # Panics
///
/// Always panics with message "unsupported data type: {name}"
///
/// # Example
///
/// ```ignore
/// // This would panic with "unsupported data type: [number NaN]"
/// throw_unsupported_data("[number NaN]");
/// ```
pub fn throw_unsupported_data(name: &str) -> ! {
    panic!("unsupported data type: {name}");
}
