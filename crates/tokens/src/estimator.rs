//! Fast heuristic token estimator.
//!
//! For when you don't need exact counts — estimates tokens as ~3.5 chars per token.
//! Accurate to within ~15% for English text and code.

/// Estimate the token count for a string using the chars/3.5 heuristic.
pub fn estimate_tokens(text: &str) -> u64 {
    let chars = text.len() as f64;
    (chars / 3.5).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens() {
        // "Hello, world!" is 13 chars ≈ 4 tokens (actually 4 in cl100k_base)
        let estimate = estimate_tokens("Hello, world!");
        assert!(estimate >= 3 && estimate <= 6);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(estimate_tokens(""), 0);
    }
}
