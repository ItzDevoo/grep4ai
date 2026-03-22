//! Smart context windowing for grepit search results.
//!
//! Extracts surrounding context lines for each match, with support
//! for merging overlapping regions within the same file.

mod window;
mod merge;

pub use window::{ContextualMatch, ContextConfig, extract_context};
pub use merge::merge_overlapping;
