//! Near-duplicate result collapsing.
//!
//! Many codebases have identical lines across many files (imports, error strings).
//! This module collapses them to reduce noise in agent output.

use crate::scorer::ScoredMatch;
use std::collections::HashMap;

/// Configuration for deduplication.
pub struct DedupConfig {
    /// Minimum number of duplicates before collapsing.
    pub threshold: usize,
}

impl Default for DedupConfig {
    fn default() -> Self {
        Self { threshold: 3 }
    }
}

/// Result of deduplication.
pub struct DedupResult {
    /// Unique matches (kept).
    pub matches: Vec<ScoredMatch>,
    /// Number of duplicates that were collapsed.
    pub collapsed_count: u64,
}

/// Deduplicate scored matches by normalizing their line content.
pub fn deduplicate(matches: Vec<ScoredMatch>, config: &DedupConfig) -> DedupResult {
    if matches.is_empty() {
        return DedupResult {
            matches: Vec::new(),
            collapsed_count: 0,
        };
    }

    // Group by normalized line content
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, m) in matches.iter().enumerate() {
        let key = normalize_line(&m.raw.line_content);
        groups.entry(key).or_default().push(idx);
    }

    let mut keep_indices: Vec<usize> = Vec::new();
    let mut collapsed: u64 = 0;

    for (_key, mut indices) in groups {
        if indices.len() >= config.threshold {
            // Keep only the highest-scored match from this group
            indices.sort_by(|&a, &b| {
                matches[b]
                    .score
                    .partial_cmp(&matches[a].score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            keep_indices.push(indices[0]);
            collapsed += (indices.len() - 1) as u64;
        } else {
            keep_indices.extend(indices);
        }
    }

    // Sort to maintain original order
    keep_indices.sort();

    let kept_matches: Vec<ScoredMatch> = keep_indices
        .into_iter()
        .map(|i| matches[i].clone())
        .collect();

    DedupResult {
        matches: kept_matches,
        collapsed_count: collapsed,
    }
}

/// Normalize a line for comparison: trim whitespace and collapse runs.
fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use grepit_searcher::RawMatch;
    use std::path::PathBuf;

    fn make_scored(path: &str, line: &str, score: f32) -> ScoredMatch {
        ScoredMatch {
            raw: RawMatch {
                path: PathBuf::from(path),
                line_number: 1,
                column: 1,
                line_content: line.to_string(),
                match_text: "use".to_string(),
                file_line_count: 10,
            },
            score,
            signals: Default::default(),
        }
    }

    #[test]
    fn test_dedup_collapses_identical_lines() {
        let matches = vec![
            make_scored("a.rs", "use std::io;", 0.5),
            make_scored("b.rs", "use std::io;", 0.3),
            make_scored("c.rs", "use std::io;", 0.8),
            make_scored("d.rs", "use std::io;", 0.4),
            make_scored("e.rs", "fn main() {", 0.9),
        ];

        let config = DedupConfig { threshold: 3 };
        let result = deduplicate(matches, &config);

        assert_eq!(result.matches.len(), 2); // 1 kept from dup group + 1 unique
        assert_eq!(result.collapsed_count, 3);
    }

    #[test]
    fn test_dedup_keeps_all_below_threshold() {
        let matches = vec![
            make_scored("a.rs", "use std::io;", 0.5),
            make_scored("b.rs", "use std::io;", 0.3),
            make_scored("c.rs", "fn main() {", 0.9),
        ];

        let config = DedupConfig { threshold: 3 };
        let result = deduplicate(matches, &config);

        assert_eq!(result.matches.len(), 3);
        assert_eq!(result.collapsed_count, 0);
    }
}
