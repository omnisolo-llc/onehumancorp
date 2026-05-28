use tree_sitter::{Node, Parser};

pub struct ASTParser {
    parser: Parser,
}

impl ASTParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_bash::LANGUAGE.into())
            .expect("Error loading bash grammar");
        Self { parser }
    }

    pub fn parse_for_security(&mut self, cmd: &str) -> Result<(), String> {
        let tree = self.parser.parse(cmd, None).ok_or("Failed to parse command")?;
        let root_node = tree.root_node();

        self.walk_node_for_security(root_node, cmd)
    }

    fn walk_node_for_security(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let node_kind = node.kind();

        // Check for git internal path creation attempts (e.g. .git/HEAD, .git/objects/, .git/hooks/)
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let start_byte = command_name_node.start_byte() as usize;
                let end_byte = command_name_node.end_byte() as usize;
                let name = &source[start_byte..end_byte];
                if name == "git" || name == "echo" || name == "touch" || name == "cat" || name == "cp" || name == "mv" || name == "mkdir" {
                    // Quick text-based check in the command for git paths if it modifies files
                    let node_start_byte = node.start_byte() as usize;
                    let node_end_byte = node.end_byte() as usize;
                    let cmd_text = &source[node_start_byte..node_end_byte];
                    if cmd_text.contains(".git/HEAD") ||
                       cmd_text.contains(".git/objects/") ||
                       cmd_text.contains(".git/refs/") ||
                       cmd_text.contains(".git/hooks/") {
                        return Err("Dangerous pattern detected: git internal path write".to_string());
                    }
                }
            }
        }

        // zmodload check
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let start_byte = command_name_node.start_byte() as usize;
                let end_byte = command_name_node.end_byte() as usize;
                let name = &source[start_byte..end_byte];
                if name == "zmodload" {
                    return Err("Dangerous pattern detected: zmodload".to_string());
                }
            }
        }

        // process substitution <() or >()
        if node_kind == "process_substitution" {
            let start_byte = node.start_byte() as usize;
            let end_byte = node.end_byte() as usize;
            let text = &source[start_byte..end_byte];
            if text.starts_with("<(") {
                return Err("Dangerous pattern detected: <() process substitution".to_string());
            }
            if text.starts_with(">(") {
                return Err("Dangerous pattern detected: >() process substitution".to_string());
            }
        }

        // legacy expansions $[] (not strictly supported in all bash, often handled as expansion)
        // Check for node type "expansion" or similar, and check text.
        // In tree-sitter-bash it might be `expansion` or `arithmetic_expansion`
        if node_kind == "expansion" || node_kind == "arithmetic_expansion" {
            let start_byte = node.start_byte() as usize;
            let end_byte = node.end_byte() as usize;
            let text = &source[start_byte..end_byte];
            if text.starts_with("$[") && text.ends_with("]") {
                return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
            }
        }

        // Just in case it's not parsed properly, check string content for $[] as fallback, but only if it's text.
        // But doing it robustly:
        if node_kind == "word" || node_kind == "raw_string" {
             let start_byte = node.start_byte() as usize;
             let end_byte = node.end_byte() as usize;
             let text = &source[start_byte..end_byte];
             if text.starts_with("$[") && text.ends_with("]") {
                  return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
             }
        }


        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_node_for_security(child, source)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_commands() {
        let mut parser = ASTParser::new();
        assert!(parser.parse_for_security("echo 'hello world'").is_ok());
        assert!(parser.parse_for_security("ls -l /tmp").is_ok());
        assert!(parser.parse_for_security("cat file.txt | grep foo").is_ok());
    }

    #[test]
    fn test_block_git_internal_path_write() {
        let mut parser = ASTParser::new();

        let res = parser.parse_for_security("echo 'test' > .git/hooks/pre-commit");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: git internal path write");

        let res2 = parser.parse_for_security("touch .git/HEAD");
        assert!(res2.is_err());
        assert_eq!(res2.unwrap_err(), "Dangerous pattern detected: git internal path write");

        let res3 = parser.parse_for_security("cp file .git/objects/obj");
        assert!(res3.is_err());
        assert_eq!(res3.unwrap_err(), "Dangerous pattern detected: git internal path write");

        let res4 = parser.parse_for_security("mkdir -p .git/refs/heads");
        assert!(res4.is_err());
        assert_eq!(res4.unwrap_err(), "Dangerous pattern detected: git internal path write");
    }

    #[test]
    fn test_block_zmodload() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("zmodload zsh/net/tcp");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: zmodload");
    }

    #[test]
    fn test_block_process_substitution_out() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo 'test' > >(cat)");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[test]
    fn test_block_process_substitution_in() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("cat < <(echo 'test')");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: <() process substitution");
    }

    #[test]
    fn test_block_legacy_expansion() {
        let mut parser = ASTParser::new();
        // $[] might be parsed as word or expansion depending on tree-sitter-bash grammar rules
        let res = parser.parse_for_security("echo $[1+1]");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: $[] legacy expansion");
    }
}
