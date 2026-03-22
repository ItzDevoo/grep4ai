//! Core search engine for grepit.
//!
//! Provides parallel regex matching over files with memory-mapped I/O
//! for maximum throughput.

mod engine;
mod matcher;
mod binary;
mod filter;

pub use engine::{SearchEngine, SearchConfig};
pub use matcher::RawMatch;
pub use binary::is_binary;
pub use filter::should_skip_path;

/// Re-export walker types that flow through the search pipeline.
pub use grepit_walker::{FileEntry, FileType, classify_file_type};
