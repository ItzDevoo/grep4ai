//! grep4ai — The fastest grep tool built for AI agents.
//!
//! Usage: grep4ai [OPTIONS] <PATTERN> [PATH...]
//!
//! AI-native search with structured JSON output, relevance ranking,
//! token budget awareness, and smart context windowing.

mod app;
mod cli;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    match app::run(args) {
        Ok(match_count) => {
            // Follow grep convention: exit code 1 when no matches found
            if match_count == 0 {
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("grep4ai: {e}");
            std::process::exit(2);
        }
    }
}
