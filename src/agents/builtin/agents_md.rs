use std::fs;
use std::path::Path;

const MAX_AGENTS_MD_SIZE: usize = 32 * 1024; // 32 KiB limit for OpenAI Codex Mechanic

/// Traverses from the repository root down to the given working directory,
/// collecting and concatenating `AGENTS.md` files. Deeply-nested files
/// take precedence (appear later in the text). The final output is capped
/// at 32 KiB to prevent context rot, truncating from the top if necessary.
pub fn load_cascading_agents_md(working_dir: &Path) -> std::io::Result<String> {
    // Determine the repository root. For simplicity, we just walk up
    // from working_dir until we find a directory with `.git` or `WORKSPACE` or `BUILD.bazel`.
    // If not found, we use the root of the file system (or just the current directory as root).
    let mut root_dir = working_dir.to_path_buf();
    let mut current = working_dir;

    while let Some(parent) = current.parent() {
        if parent.join(".git").exists() || parent.join("WORKSPACE").exists() || parent.join("BUILD.bazel").exists() {
            root_dir = parent.to_path_buf();
            break;
        }
        current = parent;
    }

    // Now collect all directories from root_dir down to working_dir
    let mut path_stack = Vec::new();
    let mut current_dir = working_dir.to_path_buf();

    // We must ensure current_dir starts with root_dir
    if current_dir.starts_with(&root_dir) {
        loop {
            path_stack.push(current_dir.clone());
            if current_dir == root_dir {
                break;
            }
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            } else {
                break; // Should not happen if starts_with is true
            }
        }
    } else {
        // Fallback: just use the working dir if we can't find a proper root relationship
        path_stack.push(working_dir.to_path_buf());
    }

    path_stack.reverse(); // Now from root to leaf

    let mut combined_content = String::new();

    for dir in path_stack {
        let agents_file = dir.join("AGENTS.md");
        if agents_file.exists() && agents_file.is_file() {
            if let Ok(content) = fs::read_to_string(&agents_file) {
                combined_content.push_str(&format!("\n--- From {} ---\n", agents_file.display()));
                combined_content.push_str(&content);
                combined_content.push('\n');
            }
        }
    }

    let mut final_content = combined_content.trim().to_string();

    // Cap at 32 KiB (OpenAI Codex Mechanic)
    if final_content.len() > MAX_AGENTS_MD_SIZE {
        // Truncate from the beginning (preserve the deepest/most recent context at the end)
        let truncation_point = final_content.len() - MAX_AGENTS_MD_SIZE;
        // Find a safe character boundary to split
        let mut split_point = truncation_point;
        while split_point < final_content.len() && !final_content.is_char_boundary(split_point) {
            split_point += 1;
        }

        let truncated = &final_content[split_point..];
        final_content = format!("[...Truncated {} bytes...]\n{}", split_point, truncated);
    }

    Ok(final_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_load_cascading_agents_md() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Mock a git repo to establish root
        File::create(root.join(".git")).unwrap();

        // Create AGENTS.md at root
        let mut root_agents = File::create(root.join("AGENTS.md")).unwrap();
        root_agents.write_all(b"Root instructions\n").unwrap();

        // Create nested dir and AGENTS.md
        let nested = root.join("src").join("module");
        fs::create_dir_all(&nested).unwrap();

        let mut nested_agents = File::create(nested.join("AGENTS.md")).unwrap();
        nested_agents.write_all(b"Nested instructions\n").unwrap();

        // Test loading from nested dir
        let result = load_cascading_agents_md(&nested).unwrap();

        assert!(result.contains("Root instructions"));
        assert!(result.contains("Nested instructions"));

        // Ensure order: root first, nested later
        let root_pos = result.find("Root instructions").unwrap();
        let nested_pos = result.find("Nested instructions").unwrap();
        assert!(root_pos < nested_pos);
    }

    #[test]
    fn test_truncation_mechanic() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();
        File::create(root.join(".git")).unwrap();

        let mut root_agents = File::create(root.join("AGENTS.md")).unwrap();
        // Create a large file > 32KiB
        let large_content = "A".repeat(40 * 1024);
        root_agents.write_all(large_content.as_bytes()).unwrap();

        let nested = root.join("nested");
        fs::create_dir_all(&nested).unwrap();
        let mut nested_agents = File::create(nested.join("AGENTS.md")).unwrap();
        nested_agents.write_all(b"Important Nested Context").unwrap();

        let result = load_cascading_agents_md(&nested).unwrap();

        // It should be around 32 KiB + some preamble bytes
        assert!(result.len() <= MAX_AGENTS_MD_SIZE + 100);
        assert!(result.contains("Truncated"));
        // Deeply nested context must survive the truncation!
        assert!(result.contains("Important Nested Context"));
    }
}
