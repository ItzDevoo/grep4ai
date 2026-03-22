//! Relevance scoring and deduplication for grepit search results.
//!
//! Ranks matches by combining multiple signals to surface the most
//! relevant results first — definitions over usages, source over tests.

mod scorer;
mod signals;
pub mod dedup;

pub use scorer::{ScoredMatch, rank_matches, RankConfig};
pub use signals::SignalSet;
pub use dedup::deduplicate;
