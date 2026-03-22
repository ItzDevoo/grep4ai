//! Parallel, gitignore-aware directory traversal for grepit.
//!
//! Wraps the `ignore` crate (ripgrep's own walker) to provide
//! a high-performance file discovery pipeline.

mod walk;
mod filetype;
mod gitaware;

pub use walk::{Walker, WalkerConfig, FileEntry};
pub use filetype::{FileType, classify_file_type};
pub use gitaware::find_repo_root;
