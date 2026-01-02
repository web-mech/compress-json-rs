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
//! println!("Preserve NaN: {}", CONFIG.preserve_nan);
//! println!("Preserve Infinity: {}", CONFIG.preserve_infinite);
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
/// | `preserve_nan` | `false` | Encode NaN as `N\|0` (vs convert to null) |
/// | `error_on_nan` | `false` | Panic on NaN (only if `preserve_nan` is false) |
/// | `preserve_infinite` | `false` | Encode Infinity as `N\|+`/`N\|-` (vs convert to null) |
/// | `error_on_infinite` | `false` | Panic on Infinity (only if `preserve_infinite` is false) |
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
/// # Special Number Handling (v3.4.0+)
///
/// JSON doesn't support `NaN` or `Infinity`. The handling depends on config:
///
/// | Value | `preserve_*` = true | `preserve_*` = false, `error_*` = true | Both false |
/// |-------|---------------------|----------------------------------------|------------|
/// | NaN | Encoded as `N\|0` | Panic | Becomes `null` |
/// | Infinity | Encoded as `N\|+` | Panic | Becomes `null` |
/// | -Infinity | Encoded as `N\|-` | Panic | Becomes `null` |
///
/// Note: `error_on_nan` and `error_on_infinite` only take effect when
/// their corresponding `preserve_*` option is `false`.
///
/// # Example
///
/// ```rust
/// use compress_json_rs::{Config, CONFIG};
///
/// // View the default configuration
/// assert_eq!(CONFIG.sort_key, false);
/// assert_eq!(CONFIG.preserve_nan, false);
/// assert_eq!(CONFIG.error_on_nan, false);
/// assert_eq!(CONFIG.preserve_infinite, false);
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

    /// Whether to preserve NaN values with special encoding.
    ///
    /// When `true`, NaN values are encoded as `N|0` for cross-platform
    /// compatibility with JavaScript/Python implementations.
    /// When `false`, NaN is converted to `null` (like `JSON.stringify`).
    ///
    /// **Default:** `false`
    ///
    /// **Added in:** v3.4.0
    pub preserve_nan: bool,

    /// Whether to panic when encountering NaN values.
    ///
    /// Only effective when `preserve_nan` is `false`.
    /// When `true`, the library will panic if a NaN is encountered.
    /// When `false`, NaN is silently converted to `null`.
    ///
    /// **Default:** `false`
    pub error_on_nan: bool,

    /// Whether to preserve infinite values with special encoding.
    ///
    /// When `true`, `Infinity` and `-Infinity` are encoded as `N|+` and `N|-`
    /// for cross-platform compatibility with JavaScript/Python implementations.
    /// When `false`, infinite values are converted to `null` (like `JSON.stringify`).
    ///
    /// **Default:** `false`
    ///
    /// **Added in:** v3.4.0
    pub preserve_infinite: bool,

    /// Whether to panic when encountering infinite values.
    ///
    /// Only effective when `preserve_infinite` is `false`.
    /// When `true`, the library will panic if `Infinity` or `-Infinity` is encountered.
    /// When `false`, infinite values are silently converted to `null`.
    ///
    /// **Default:** `false`
    pub error_on_infinite: bool,
}

/// Default configuration matching the TypeScript implementation.
///
/// This constant provides the default behavior for compression:
/// - Object keys maintain their original order
/// - NaN values become `null` (like `JSON.stringify`)
/// - Infinity values become `null` (like `JSON.stringify`)
///
/// To preserve special values, set `preserve_nan` and/or `preserve_infinite` to `true`.
///
/// # Values
///
/// ```rust
/// use compress_json_rs::CONFIG;
///
/// // All options default to false
/// assert!(!CONFIG.sort_key);
/// assert!(!CONFIG.preserve_nan);
/// assert!(!CONFIG.error_on_nan);
/// assert!(!CONFIG.preserve_infinite);
/// assert!(!CONFIG.error_on_infinite);
/// ```
///
/// # Compatibility
///
/// These defaults match the JavaScript [compress-json](https://github.com/beenotung/compress-json)
/// library v3.4.0+, ensuring cross-platform compatibility.
pub const CONFIG: Config = Config {
    sort_key: false,
    preserve_nan: false,
    error_on_nan: false,
    preserve_infinite: false,
    error_on_infinite: false,
};
