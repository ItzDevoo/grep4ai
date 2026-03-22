//! Token counter implementation.
//!
//! Uses the fast heuristic estimator by default.
//! A future `precise-tokens` feature will add tiktoken support.

use crate::estimator::estimate_tokens;

/// Token counter that can count tokens in text.
#[derive(Debug, Clone)]
pub struct TokenCounter {
    /// Total tokens counted so far.
    total: u64,
}

impl TokenCounter {
    pub fn new() -> Self {
        Self { total: 0 }
    }

    /// Count tokens in the given text and add to the running total.
    pub fn count(&mut self, text: &str) -> u64 {
        let tokens = estimate_tokens(text);
        self.total += tokens;
        tokens
    }

    /// Get the current total token count.
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Reset the counter.
    pub fn reset(&mut self) {
        self.total = 0;
    }
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_accumulates() {
        let mut counter = TokenCounter::new();
        counter.count("hello world");
        counter.count("foo bar");
        assert!(counter.total() > 0);
    }
}
