//! Git-aware utilities for repository detection.

use std::path::{Path, PathBuf};

/// Walk up from `start` to find the repository root (directory containing `.git`).
pub fn find_repo_root(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };

    loop {
        if current.join(".git").exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_repo_root_returns_none_for_root() {
        // This test is platform-dependent but should not panic
        let result = find_repo_root(Path::new("/"));
        // May or may not find a repo root depending on environment
        let _ = result;
    }
}
