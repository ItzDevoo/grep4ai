//! File type detection and classification.

use std::path::Path;

/// Broad file type categories for scoring and filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    C,
    Cpp,
    Ruby,
    Shell,
    Markdown,
    Json,
    Yaml,
    Toml,
    Html,
    Css,
    Sql,
    Proto,
    Dockerfile,
    Unknown,
}

impl FileType {
    /// Returns the canonical name used in CLI flags (e.g., `--type rust`).
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Go => "go",
            Self::Java => "java",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Ruby => "ruby",
            Self::Shell => "shell",
            Self::Markdown => "markdown",
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
            Self::Html => "html",
            Self::Css => "css",
            Self::Sql => "sql",
            Self::Proto => "protobuf",
            Self::Dockerfile => "dockerfile",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this is a source code file (vs config/docs/data).
    pub fn is_source(&self) -> bool {
        matches!(
            self,
            Self::Rust
                | Self::Python
                | Self::JavaScript
                | Self::TypeScript
                | Self::Go
                | Self::Java
                | Self::C
                | Self::Cpp
                | Self::Ruby
                | Self::Shell
                | Self::Sql
                | Self::Proto
        )
    }
}

/// Classify a file by its extension.
pub fn classify_file_type(path: &Path) -> FileType {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");

    match ext.to_lowercase().as_str() {
        "rs" => FileType::Rust,
        "py" | "pyi" | "pyw" => FileType::Python,
        "js" | "mjs" | "cjs" | "jsx" => FileType::JavaScript,
        "ts" | "mts" | "cts" | "tsx" => FileType::TypeScript,
        "go" => FileType::Go,
        "java" => FileType::Java,
        "c" | "h" => FileType::C,
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => FileType::Cpp,
        "rb" | "erb" => FileType::Ruby,
        "sh" | "bash" | "zsh" | "fish" => FileType::Shell,
        "md" | "mdx" | "rst" => FileType::Markdown,
        "json" | "jsonc" | "json5" => FileType::Json,
        "yml" | "yaml" => FileType::Yaml,
        "toml" => FileType::Toml,
        "html" | "htm" => FileType::Html,
        "css" | "scss" | "sass" | "less" => FileType::Css,
        "sql" => FileType::Sql,
        "proto" => FileType::Proto,
        _ => {
            // Check special filenames
            let lower = filename.to_lowercase();
            if lower == "dockerfile" || lower.starts_with("dockerfile.") {
                FileType::Dockerfile
            } else {
                FileType::Unknown
            }
        }
    }
}

/// Resolve a user-provided type alias to the canonical FileType name.
///
/// Accepts both short aliases (`js`, `ts`, `py`, `rb`, `sh`, `md`, `yml`)
/// and canonical names (`javascript`, `typescript`, `python`, etc.).
pub fn resolve_type_alias(alias: &str) -> Option<&'static str> {
    match alias.to_lowercase().as_str() {
        // Canonical names
        "rust" => Some("rust"),
        "python" => Some("python"),
        "javascript" => Some("javascript"),
        "typescript" => Some("typescript"),
        "go" => Some("go"),
        "java" => Some("java"),
        "c" => Some("c"),
        "cpp" | "c++" | "cxx" => Some("cpp"),
        "ruby" => Some("ruby"),
        "shell" | "bash" => Some("shell"),
        "markdown" => Some("markdown"),
        "json" => Some("json"),
        "yaml" => Some("yaml"),
        "toml" => Some("toml"),
        "html" => Some("html"),
        "css" => Some("css"),
        "sql" => Some("sql"),
        "protobuf" | "proto" => Some("protobuf"),
        "dockerfile" | "docker" => Some("dockerfile"),
        // Short aliases
        "rs" => Some("rust"),
        "py" => Some("python"),
        "js" | "jsx" | "mjs" | "cjs" => Some("javascript"),
        "ts" | "tsx" | "mts" | "cts" => Some("typescript"),
        "rb" => Some("ruby"),
        "sh" | "zsh" | "fish" => Some("shell"),
        "md" | "mdx" => Some("markdown"),
        "yml" => Some("yaml"),
        "htm" => Some("html"),
        "scss" | "sass" | "less" => Some("css"),
        _ => None,
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_classify_common_extensions() {
        assert_eq!(
            classify_file_type(&PathBuf::from("main.rs")),
            FileType::Rust
        );
        assert_eq!(
            classify_file_type(&PathBuf::from("app.py")),
            FileType::Python
        );
        assert_eq!(
            classify_file_type(&PathBuf::from("index.tsx")),
            FileType::TypeScript
        );
        assert_eq!(
            classify_file_type(&PathBuf::from("Makefile")),
            FileType::Unknown
        );
    }

    #[test]
    fn test_dockerfile_detection() {
        assert_eq!(
            classify_file_type(&PathBuf::from("Dockerfile")),
            FileType::Dockerfile
        );
        assert_eq!(
            classify_file_type(&PathBuf::from("Dockerfile.prod")),
            FileType::Dockerfile
        );
    }

    #[test]
    fn test_resolve_type_alias_short_names() {
        assert_eq!(resolve_type_alias("js"), Some("javascript"));
        assert_eq!(resolve_type_alias("ts"), Some("typescript"));
        assert_eq!(resolve_type_alias("py"), Some("python"));
        assert_eq!(resolve_type_alias("rb"), Some("ruby"));
        assert_eq!(resolve_type_alias("sh"), Some("shell"));
        assert_eq!(resolve_type_alias("rs"), Some("rust"));
        assert_eq!(resolve_type_alias("md"), Some("markdown"));
        assert_eq!(resolve_type_alias("yml"), Some("yaml"));
    }

    #[test]
    fn test_resolve_type_alias_canonical_names() {
        assert_eq!(resolve_type_alias("javascript"), Some("javascript"));
        assert_eq!(resolve_type_alias("typescript"), Some("typescript"));
        assert_eq!(resolve_type_alias("python"), Some("python"));
        assert_eq!(resolve_type_alias("rust"), Some("rust"));
    }

    #[test]
    fn test_resolve_type_alias_unknown() {
        assert_eq!(resolve_type_alias("fortran"), None);
        assert_eq!(resolve_type_alias("cobol"), None);
    }

    #[test]
    fn test_is_source() {
        assert!(FileType::Rust.is_source());
        assert!(FileType::Python.is_source());
        assert!(!FileType::Markdown.is_source());
        assert!(!FileType::Json.is_source());
    }
}
