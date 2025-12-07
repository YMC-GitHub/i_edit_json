//! A lightweight, high-performance JSON field extraction and manipulation tool.
//!
//! This library provides functionality to read and modify JSON files through
//! intuitive field paths, supporting nested structures, arrays, and type-aware
//! value handling. It can be used both as a standalone CLI tool and as a library
//! in other Rust projects.

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Error types and handling for JSON operations.
pub mod error;
pub use error::JsonExtractError;

pub mod get;
pub mod set;

// Re-export core types for convenience
pub use get::types::ExtractConfig;
pub use set::types::SetConfig;
