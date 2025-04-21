/// Global configuration for compression behavior
#[derive(Debug, Copy, Clone)]
pub struct Config {
    /// Whether to sort object keys
    pub sort_key: bool,
    /// Whether to error on NaN values
    pub error_on_nan: bool,
    /// Whether to error on infinite values
    pub error_on_infinite: bool,
}

/// Default configuration matching the TypeScript defaults
pub const CONFIG: Config = Config {
    sort_key: false,
    error_on_nan: false,
    error_on_infinite: false,
};