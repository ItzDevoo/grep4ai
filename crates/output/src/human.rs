//! Human-readable colored output.
//!
//! Looks similar to ripgrep's default output for debugging/manual use.

use std::io::Write;
use grepit_context::ContextualMatch;
use crate::OutputConfig;

/// Write human-readable output with file headers and line numbers.
pub fn write_human<W: Write>(
    writer: &mut W,
    matches: Vec<ContextualMatch>,
    files_searched: u64,
    total_matches: u64,
    duration_ms: u64,
    config: &OutputConfig,
) -> anyhow::Result<()> {
    let mut current_file: Option<String> = None;

    for m in &matches {
        let path = m.scored.raw.path.to_string_lossy().to_string();

        // Print file header when we switch files
        if current_file.as_ref() != Some(&path) {
            if current_file.is_some() {
                writeln!(writer)?;
            }
            writeln!(writer, "\x1b[35m{}\x1b[0m", path)?;
            current_file = Some(path);
        }

        // Print context before
        for (i, line) in m.context_before.iter().enumerate() {
            let line_num = m.scored.raw.line_number as i64 - m.context_before.len() as i64 + i as i64;
            writeln!(writer, "\x1b[32m{}\x1b[0m-{}", line_num, line)?;
        }

        // Print the matched line
        writeln!(
            writer,
            "\x1b[32m{}\x1b[0m:\x1b[1m\x1b[31m{}\x1b[0m",
            m.scored.raw.line_number,
            m.scored.raw.line_content,
        )?;

        // Print context after
        for (i, line) in m.context_after.iter().enumerate() {
            let line_num = m.scored.raw.line_number + 1 + i as u64;
            writeln!(writer, "\x1b[32m{}\x1b[0m-{}", line_num, line)?;
        }
    }

    if config.show_stats {
        writeln!(writer)?;
        writeln!(
            writer,
            "\x1b[36m{} matches across {} files in {}ms\x1b[0m",
            total_matches, files_searched, duration_ms
        )?;
    }

    Ok(())
}
