// Module declarations
mod number;
mod encode;
mod boolean;
mod config;
mod debug;
mod memory;
mod helpers;
mod core;

// Re-export core functionality
pub use core::{compress, decompress, Compressed, decode};

// Expose lower-level APIs
pub use memory::{add_value, make_memory, mem_to_values, Key};
pub use helpers::{trim_undefined, trim_undefined_recursively};
pub use config::CONFIG;
