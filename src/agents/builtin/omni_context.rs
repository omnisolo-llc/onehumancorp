use std::fs;
use std::path::PathBuf;

/// Omni-Context Sub-agent Routing
/// Automatically reads project-level grounding (AGENTS.md / CLAUDE.md)
/// and injects it into task context.
pub struct OmniContextRouter {
    context_root: PathBuf,
}

impl OmniContextRouter {
    pub fn new(context_root: impl Into<PathBuf>) -> Self {
        Self {
            context_root: context_root.into(),
        }
    }

    /// Reads AGENTS.md or CLAUDE.md (prioritizing AGENTS.md)
    /// Returns the content with the [SYSTEM GROUNDING] prefix.
    pub fn get_system_grounding(&self) -> Option<String> {
        let agents_path = self.context_root.join("AGENTS.md");
        if agents_path.exists()
            && let Ok(content) = fs::read_to_string(&agents_path) {
                return Some(format!("[SYSTEM GROUNDING]\n{}", content));
            }

        let claude_path = self.context_root.join("CLAUDE.md");
        if claude_path.exists()
            && let Ok(content) = fs::read_to_string(&claude_path) {
                return Some(format!("[SYSTEM GROUNDING]\n{}", content));
            }

        None
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
