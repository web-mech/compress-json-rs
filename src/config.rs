//! Configuration options for compression behavior.
//!
//! This module provides the [`Config`] struct and the global [`CONFIG`] constant
//! that controls how JSON values are compressed.
//!
//! # Current Limitations
//!
//! Configuration is currently compile-time only via the [`CONFIG`] constant.
//! Runtime configuration may be added in future versions.
//!
//! # Example
//!
//! ```rust
//! use compress_json_rs::CONFIG;
//!
//! // Check current configuration
//! println!("Sort keys: {}", CONFIG.sort_key);
//! println!("Error on NaN: {}", CONFIG.error_on_nan);
//! println!("Error on Infinity: {}", CONFIG.error_on_infinite);
//! ```

/// Global configuration for compression behavior.
///
/// This struct defines options that control how JSON values are processed
/// during compression. The library uses a compile-time constant [`CONFIG`]
/// with these settings.
///
/// # Fields
///
/// | Field | Default | Description |
/// |-------|---------|-------------|
/// | `sort_key` | `false` | Sort object keys alphabetically |
/// | `error_on_nan` | `false` | Panic on NaN values (vs convert to null) |
/// | `error_on_infinite` | `false` | Panic on Infinity values (vs convert to null) |
///
/// # Key Sorting
///
/// When `sort_key` is `true`, object keys are sorted alphabetically before
/// compression. This ensures consistent output regardless of insertion order,
/// which is useful for:
/// - Deterministic compression output
/// - Easier diff comparison
/// - Consistent hashing of compressed data
///
/// # Special Number Handling
///
/// JSON doesn't support `NaN` or `Infinity`. By default, these values are
/// silently converted to `null`. Set `error_on_nan` or `error_on_infinite`
/// to `true` to panic instead.
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{Config, CONFIG};
///
/// // View the default configuration
/// assert_eq!(CONFIG.sort_key, false);
/// assert_eq!(CONFIG.error_on_nan, false);
/// assert_eq!(CONFIG.error_on_infinite, false);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Config {
    /// Whether to sort object keys alphabetically.
    ///
    /// When `true`, object keys are sorted before compression, ensuring
    /// consistent output regardless of key insertion order.
    ///
    /// **Default:** `false`
    pub sort_key: bool,

    /// Whether to panic when encountering NaN values.
    ///
    /// When `false` (default), NaN values are silently converted to `null`.
    /// When `true`, the library will panic if a NaN is encountered.
    ///
    /// **Default:** `false`
    pub error_on_nan: bool,

    /// Whether to panic when encountering infinite values.
    ///
    /// When `false` (default), `Infinity` and `-Infinity` are silently
    /// converted to `null`. When `true`, the library will panic.
    ///
    /// **Default:** `false`
    pub error_on_infinite: bool,
}

/// Default configuration matching the TypeScript implementation.
///
/// This constant provides the default behavior for compression:
/// - Object keys maintain their original order
/// - NaN values become `null`
/// - Infinity values become `null`
///
/// # Values
///
/// ```rust
/// use compress_json_rs::CONFIG;
///
/// // All options default to false
/// assert!(!CONFIG.sort_key);
/// assert!(!CONFIG.error_on_nan);
/// assert!(!CONFIG.error_on_infinite);
/// ```
///
/// # Compatibility
///
/// These defaults match the JavaScript [compress-json](https://github.com/beenotung/compress-json)
/// library, ensuring cross-platform compatibility.
pub const CONFIG: Config = Config {
    sort_key: false,
    error_on_nan: false,
    error_on_infinite: false,
};
