use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriageCategory {
    Bug,
    Feature,
    Refactor,
    Cleanup,
    Docs,
    Security,
}

impl fmt::Display for TriageCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TriageCategory::Bug => "BUG",
            TriageCategory::Feature => "FEATURE",
            TriageCategory::Refactor => "REFACTOR",
            TriageCategory::Cleanup => "CLEANUP",
            TriageCategory::Docs => "DOCS",
            TriageCategory::Security => "SECURITY",
        };
        write!(f, "{}", s)
    }
}

pub fn triage_log(category: TriageCategory, message: &str) {
    let timestamp = chrono::Utc::now().to_rfc3339();

    // OHC Premium Formatting (Simulated with ANSI colors and structured layout)
    // Glassmorphism/Premium feel is hard in text logs, but we can use consistent headers.
    println!(
        "\x1b[1;34m[OHC-TRIAGE]\x1b[0m \x1b[36m{}\x1b[0m [\x1b[1;33m{}\x1b[0m] - {}",
        timestamp,
        category,
        message
    );
}

pub fn triage_error(category: TriageCategory, message: &str) {
    let timestamp = chrono::Utc::now().to_rfc3339();
    eprintln!(
        "\x1b[1;31m[OHC-TRIAGE-ERROR]\x1b[0m \x1b[36m{}\x1b[0m [\x1b[1;33m{}\x1b[0m] - {}",
        timestamp,
        category,
        message
    );
}
