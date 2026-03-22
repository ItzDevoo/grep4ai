//! Binary file detection.
//!
//! Uses a simple heuristic: if the first N bytes contain a NUL byte,
//! the file is likely binary. This matches ripgrep's approach.

const BINARY_CHECK_SIZE: usize = 8192;

/// Check if a byte slice looks like binary content.
/// Returns true if a NUL byte is found in the first 8KB.
pub fn is_binary(data: &[u8]) -> bool {
    let check_len = data.len().min(BINARY_CHECK_SIZE);
    data[..check_len].contains(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_not_binary() {
        assert!(!is_binary(b"Hello, world!\nThis is text.\n"));
    }

    #[test]
    fn test_binary_with_null() {
        assert!(is_binary(b"Hello\x00world"));
    }

    #[test]
    fn test_empty_not_binary() {
        assert!(!is_binary(b""));
    }
}
