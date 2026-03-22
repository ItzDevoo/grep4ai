//! Regex matching with position tracking.
//!
//! Uses byte-oriented line scanning to avoid collecting all lines into a Vec.
//! We count newlines with memchr for the total line count, then iterate lines
//! with a streaming scan — no intermediate allocation for non-matching lines.

use std::path::{Path, PathBuf};

/// A single match found in a file.
#[derive(Debug, Clone)]
pub struct RawMatch {
    /// Path to the file containing the match.
    pub path: PathBuf,
    /// 1-based line number.
    pub line_number: u64,
    /// 1-based column number (byte offset within the line).
    pub column: u64,
    /// The full content of the matched line (trimmed of trailing newline).
    pub line_content: String,
    /// The actual matched text (the substring that matched the pattern).
    pub match_text: String,
    /// Total number of lines in the file (for context extraction).
    pub file_line_count: u64,
}

/// Search a file's content (as bytes) for matches against a compiled regex.
/// Returns all matches found.
///
/// Uses streaming line iteration — never collects all lines into a Vec.
/// Only allocates strings for lines that actually match.
pub fn find_matches(
    path: &Path,
    content: &[u8],
    regex: &regex::Regex,
    max_count: Option<usize>,
) -> Vec<RawMatch> {
    // Validate UTF-8 once upfront (zero-copy check)
    let text = match std::str::from_utf8(content) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    // Count total lines via fast newline counting (memchr-optimized)
    let file_line_count = memchr::memchr_iter(b'\n', content).count() as u64 + 1;

    let mut matches = Vec::new();
    let path_buf = path.to_path_buf();

    // Stream through lines — no collect(), no Vec<&str> allocation
    for (line_idx, line) in text.lines().enumerate() {
        if let Some(max) = max_count {
            if matches.len() >= max {
                break;
            }
        }

        if let Some(m) = regex.find(line) {
            matches.push(RawMatch {
                path: path_buf.clone(),
                line_number: (line_idx + 1) as u64,
                column: (m.start() + 1) as u64,
                line_content: line.to_string(),
                match_text: m.as_str().to_string(),
                file_line_count,
            });
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches_basic() {
        let regex = regex::Regex::new("hello").unwrap();
        let content = b"say hello world\ngoodbye\nhello again";
        let path = PathBuf::from("test.txt");
        let matches = find_matches(&path, content, &regex, None);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_number, 1);
        assert_eq!(matches[0].column, 5);
        assert_eq!(matches[0].match_text, "hello");
        assert_eq!(matches[1].line_number, 3);
    }

    #[test]
    fn test_find_matches_max_count() {
        let regex = regex::Regex::new("a").unwrap();
        let content = b"a\na\na\na";
        let path = PathBuf::from("test.txt");
        let matches = find_matches(&path, content, &regex, Some(2));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_find_matches_no_match() {
        let regex = regex::Regex::new("xyz").unwrap();
        let content = b"hello world";
        let path = PathBuf::from("test.txt");
        let matches = find_matches(&path, content, &regex, None);
        assert!(matches.is_empty());
    }
}
