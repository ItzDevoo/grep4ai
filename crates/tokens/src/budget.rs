//! Token budget enforcement.
//!
//! Takes a stream of serialized results and ensures the total
//! output stays within a token budget.

use crate::counter::TokenCounter;

/// Result of applying a token budget to output.
pub struct BudgetResult {
    /// The output that fits within the budget.
    pub output: String,
    /// Total tokens used.
    pub tokens_used: u64,
    /// Whether results were truncated.
    pub truncated: bool,
    /// Number of results included.
    pub results_included: usize,
}

/// Enforces a token budget on serialized output.
pub struct BudgetEnforcer {
    budget: u64,
    counter: TokenCounter,
}

impl BudgetEnforcer {
    pub fn new(budget: u64) -> Self {
        Self {
            budget,
            counter: TokenCounter::new(),
        }
    }

    /// Check if adding `text` would exceed the budget.
    pub fn would_exceed(&self, text: &str) -> bool {
        let estimate = crate::estimator::estimate_tokens(text);
        self.counter.total() + estimate > self.budget
    }

    /// Add text to the budget. Returns false if it would exceed.
    pub fn try_add(&mut self, text: &str) -> bool {
        if self.would_exceed(text) {
            return false;
        }
        self.counter.count(text);
        true
    }

    /// Get tokens used so far.
    pub fn tokens_used(&self) -> u64 {
        self.counter.total()
    }

    /// Get the configured budget.
    pub fn budget(&self) -> u64 {
        self.budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_enforcer() {
        let mut enforcer = BudgetEnforcer::new(10);

        // Short text should fit
        assert!(enforcer.try_add("hi"));
        assert!(!enforcer.would_exceed("ok"));

        // Eventually should exceed
        let long_text = "a".repeat(100);
        assert!(!enforcer.try_add(&long_text));
    }
}
