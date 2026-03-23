//! Parallel, gitignore-aware directory traversal for grep4ai.
//!
//! Wraps the `ignore` crate (ripgrep's own walker) to provide
//! a high-performance file discovery pipeline.

mod filetype;
mod gitaware;
mod walk;

pub use filetype::{classify_file_type, resolve_type_alias, FileType};
pub use gitaware::find_repo_root;
pub use walk::{FileEntry, Walker, WalkerConfig};
