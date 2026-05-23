use std::path::PathBuf;
use std::fs;
use std::io;

/// DeerFlow Unique Harness Innovations: Progressive skills: Markdown-based skills loaded progressively.
/// Represents a single progressive skill loaded from a Markdown file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressiveSkill {
    pub name: String,
    pub keywords: Vec<String>,
    pub instruction: String,
}

pub struct ProgressiveSkillManager {
    pub skills_dir: PathBuf,
}

impl ProgressiveSkillManager {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self { skills_dir }
    }

    /// Loads all skills from the given directory.
    pub fn load_all_skills(&self) -> io::Result<Vec<ProgressiveSkill>> {
        let mut skills = Vec::new();
        if !self.skills_dir.exists() || !self.skills_dir.is_dir() {
            return Ok(skills);
        }

        for entry in fs::read_dir(&self.skills_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(skill) = Self::parse_markdown_skill(&content) {
                        skills.push(skill);
                    }
                }
            }
        }
        Ok(skills)
    }

    /// Parses a single Markdown file into a ProgressiveSkill.
    /// Expects format:
    /// # Skill Name
    /// Keywords: comma, separated, list
    ///
    /// Rest of the instruction...
    pub fn parse_markdown_skill(content: &str) -> Option<ProgressiveSkill> {
        let mut lines = content.lines();

        let mut name = String::new();
        let mut keywords = Vec::new();
        let mut instruction_lines = Vec::new();

        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() && name.is_empty() {
                continue;
            }

            if name.is_empty() && line.starts_with("# ") {
                name = line[2..].trim().to_string();
            } else if line.to_lowercase().starts_with("keywords:") {
                let kw_str = line["keywords:".len()..].trim();
                keywords = kw_str.split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect();
            } else {
                instruction_lines.push(line);
            }
        }

        if name.is_empty() {
            return None;
        }

        let instruction = instruction_lines.join("\n").trim().to_string();

        Some(ProgressiveSkill {
            name,
            keywords,
            instruction,
        })
    }

    /// Returns the relevant skills based on matching the task text against the skill keywords.
    pub fn get_relevant_skills(&self, task: &str) -> io::Result<Vec<ProgressiveSkill>> {
        let all_skills = self.load_all_skills()?;
        let task_lower = task.to_lowercase();

        let mut relevant = Vec::new();
        for skill in all_skills {
            let is_match = skill.keywords.iter().any(|kw| task_lower.contains(kw));
            if is_match {
                relevant.push(skill);
            }
        }
        Ok(relevant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_markdown_skill() {
        let content = "# Code Review Skill\nKeywords: review, git, diff\n\nAlways check for off-by-one errors.";
        let skill = ProgressiveSkillManager::parse_markdown_skill(content).unwrap();
        assert_eq!(skill.name, "Code Review Skill");
        assert_eq!(skill.keywords, vec!["review", "git", "diff"]);
        assert_eq!(skill.instruction, "Always check for off-by-one errors.");
    }

    #[test]
    fn test_get_relevant_skills() {
        let dir = tempdir().unwrap();
        let skills_dir = dir.path().join("skills");
        fs::create_dir(&skills_dir).unwrap();

        let review_md = skills_dir.join("review.md");
        fs::write(&review_md, "# Review\nKeywords: review, diff\n\nReview code.").unwrap();

        let refactor_md = skills_dir.join("refactor.md");
        fs::write(&refactor_md, "# Refactor\nKeywords: refactor, cleanup\n\nRefactor code.").unwrap();

        let manager = ProgressiveSkillManager::new(skills_dir);
        let relevant = manager.get_relevant_skills("Please do a review of this PR").unwrap();

        assert_eq!(relevant.len(), 1);
        assert_eq!(relevant[0].name, "Review");
    }
}
