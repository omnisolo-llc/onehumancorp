use crate::proto::agent_service::SkillConfig;
use std::path::Path;
use tokio::fs;

/// DeerFlow Unique Harness Innovations: Progressive skills
/// Markdown-based skills loaded progressively
pub async fn load_progressive_skills(dir: &Path) -> Vec<SkillConfig> {
    let mut skills = Vec::new();

    if !dir.exists() || !dir.is_dir() {
        return skills;
    }

    let mut entries = match fs::read_dir(dir).await {
        Ok(e) => e,
        Err(_) => return skills,
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Ok(content) = fs::read_to_string(&path).await {
                if let Some(skill) = parse_markdown_skill(&content) {
                    skills.push(skill);
                }
            }
        }
    }

    skills
}

fn parse_markdown_skill(content: &str) -> Option<SkillConfig> {
    let mut name = String::new();
    let mut description = String::new();
    let mut instruction = String::new();
    let mut allowed_tools = Vec::new();
    let mut model = String::new();

    let mut current_section = "";

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("# Name:") || line.starts_with("Name:") {
            name = line.trim_start_matches("# Name:").trim_start_matches("Name:").trim().to_string();
            current_section = "name";
        } else if line.starts_with("# Description:") || line.starts_with("Description:") {
            description = line.trim_start_matches("# Description:").trim_start_matches("Description:").trim().to_string();
            current_section = "description";
        } else if line.starts_with("# Instruction:") || line.starts_with("Instruction:") {
            instruction = line.trim_start_matches("# Instruction:").trim_start_matches("Instruction:").trim().to_string();
            current_section = "instruction";
        } else if line.starts_with("# Allowed Tools:") || line.starts_with("Allowed Tools:") {
            let tools_str = line.trim_start_matches("# Allowed Tools:").trim_start_matches("Allowed Tools:").trim();
            allowed_tools = tools_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            current_section = "allowed_tools";
        } else if line.starts_with("# Model:") || line.starts_with("Model:") {
            model = line.trim_start_matches("# Model:").trim_start_matches("Model:").trim().to_string();
            current_section = "model";
        } else if !line.is_empty() {
            // Append to current section if multi-line
            match current_section {
                "description" => {
                    if !description.is_empty() {
                        description.push('\n');
                    }
                    description.push_str(line);
                }
                "instruction" => {
                    if !instruction.is_empty() {
                        instruction.push('\n');
                    }
                    instruction.push_str(line);
                }
                _ => {}
            }
        }
    }

    if name.is_empty() || instruction.is_empty() {
        return None;
    }

    Some(SkillConfig {
        name,
        description,
        instruction,
        allowed_tools,
        model,
        toolset: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_parsing() {
        let content = "
# Name: Code Reviewer
# Description: Reviews code for security issues.
# Allowed Tools: grep, read
# Model: gpt-4
# Instruction:
You must review the code carefully.
Pay attention to SQL injections.
";
        let skill = parse_markdown_skill(content).unwrap();
        assert_eq!(skill.name, "Code Reviewer");
        assert_eq!(skill.description, "Reviews code for security issues.");
        assert_eq!(skill.allowed_tools, vec!["grep", "read"]);
        assert_eq!(skill.model, "gpt-4");
        assert_eq!(skill.instruction, "You must review the code carefully.\nPay attention to SQL injections.");
    }

    #[test]
    fn test_missing_fields() {
        // Missing instruction
        let content1 = "
# Name: Incomplete Skill
# Description: No instruction
";
        assert!(parse_markdown_skill(content1).is_none());

        // Missing name
        let content2 = "
# Description: No name
# Instruction: Do something
";
        assert!(parse_markdown_skill(content2).is_none());

        // Only name and instruction, others default to empty
        let content3 = "
# Name: Minimal Skill
# Instruction: Do something
";
        let skill = parse_markdown_skill(content3).unwrap();
        assert_eq!(skill.name, "Minimal Skill");
        assert_eq!(skill.description, "");
        assert_eq!(skill.instruction, "Do something");
        assert!(skill.allowed_tools.is_empty());
        assert_eq!(skill.model, "");
    }
}
