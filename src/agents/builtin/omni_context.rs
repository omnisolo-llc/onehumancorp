#![allow(clippy::empty_line_after_doc_comments)]
use std::fs;
use std::path::PathBuf;

/// Omni-Context Sub-agent Routing
/// Automatically reads project-level grounding (AGENTS.md / CLAUDE.md)
/// and injects it into task context.

<<<<<<< HEAD
pub struct OmniContextRouter {
    context_root: PathBuf,
=======
use std::sync::RwLock;
use std::time::{Instant, Duration};

struct CacheEntry {
    content: Option<String>,
    timestamp: Instant,
}

pub struct OmniContextRouter {
    context_root: PathBuf,
    cached_grounding: RwLock<Option<CacheEntry>>,
    ttl: Duration,
>>>>>>> fe5b90eb (fix(auth): add v5 feature to uuid crate dependency in Cargo.toml files)
}

impl OmniContextRouter {
    pub fn new(context_root: impl Into<PathBuf>) -> Self {
        Self {
            context_root: context_root.into(),
<<<<<<< HEAD
=======
            cached_grounding: RwLock::new(None),
            ttl: Duration::from_secs(5), // 5 seconds TTL
>>>>>>> fe5b90eb (fix(auth): add v5 feature to uuid crate dependency in Cargo.toml files)
        }
    }

    /// Reads AGENTS.md or CLAUDE.md (prioritizing AGENTS.md)
    /// Returns the content with the [SYSTEM GROUNDING] prefix.
    pub fn get_system_grounding(&self) -> Option<String> {
<<<<<<< HEAD
        let agents_path = self.context_root.join("AGENTS.md");
        if agents_path.exists()
            && let Ok(content) = fs::read_to_string(&agents_path)
        {
            return Some(format!("[SYSTEM GROUNDING]\n{}", content));
        }

        let claude_path = self.context_root.join("CLAUDE.md");
        if claude_path.exists()
            && let Ok(content) = fs::read_to_string(&claude_path)
        {
            return Some(format!("[SYSTEM GROUNDING]\n{}", content));
        }

        None
=======
        // Check cache first (read lock)
        if let Ok(cache) = self.cached_grounding.read() {
            if let Some(entry) = &*cache {
                if entry.timestamp.elapsed() < self.ttl {
                    return entry.content.clone();
                }
            }
        }

        let mut grounding_content = None;

        let agents_path = self.context_root.join("AGENTS.md");
        if agents_path.exists() {
            if let Ok(content) = fs::read_to_string(&agents_path) {
                grounding_content = Some(format!("[SYSTEM GROUNDING]\n{}", content));
            }
        } else {
            let claude_path = self.context_root.join("CLAUDE.md");
            if claude_path.exists() {
                if let Ok(content) = fs::read_to_string(&claude_path) {
                    grounding_content = Some(format!("[SYSTEM GROUNDING]\n{}", content));
                }
            }
        }

        // Update cache (write lock)
        if let Ok(mut cache) = self.cached_grounding.write() {
            *cache = Some(CacheEntry {
                content: grounding_content.clone(),
                timestamp: Instant::now(),
            });
        }

        grounding_content
>>>>>>> fe5b90eb (fix(auth): add v5 feature to uuid crate dependency in Cargo.toml files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_no_grounding_file() {
        let dir = tempdir().unwrap();
        let router = OmniContextRouter::new(dir.path());
        assert_eq!(router.get_system_grounding(), None);
    }

    #[test]
    fn test_agents_md_injected() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("AGENTS.md"), "Always write clean code.").unwrap();
        let router = OmniContextRouter::new(dir.path());
        let grounding = router.get_system_grounding().unwrap();
        assert!(grounding.contains("[SYSTEM GROUNDING]"));
        assert!(grounding.contains("Always write clean code."));
    }

    #[test]
    fn test_claude_md_injected() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("CLAUDE.md"), "Use specialized tokens.").unwrap();
        let router = OmniContextRouter::new(dir.path());
        let grounding = router.get_system_grounding().unwrap();
        assert!(grounding.contains("[SYSTEM GROUNDING]"));
        assert!(grounding.contains("Use specialized tokens."));
    }

    #[test]
    fn test_agents_md_prioritized() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("AGENTS.md"), "AGENTS content").unwrap();
        fs::write(dir.path().join("CLAUDE.md"), "CLAUDE content").unwrap();
        let router = OmniContextRouter::new(dir.path());
        let grounding = router.get_system_grounding().unwrap();
        assert!(grounding.contains("AGENTS content"));
        assert!(!grounding.contains("CLAUDE content"));
    }
}
