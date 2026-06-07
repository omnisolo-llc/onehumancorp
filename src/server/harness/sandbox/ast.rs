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

        let old_result = self.walk_node_for_security(root_node, cmd);

        let mut parsed_command = super::bash_security::ParsedCommand::new(cmd);
        let new_result = parsed_command.parse_and_validate();

        // the prompt suggests "Note divergence where applicable"
        // Since we emit telemetry in `SandboxManager`, we can just log here or
        // rely on `SandboxManager` to handle the error. For now, we prefer the new result
        // if it caught something the old one didn't, or just return the new result entirely.
        // Returning new_result allows the more robust validators to block.
        // However, we also want to keep the old walk_node_for_security to catch
        // `>` and `<` process substitution, and `$[]` which were not explicitly asked to be removed,
        // so we'll run both and fail if either fails.

        if old_result.is_err() {
            return old_result;
        }

        new_result
    }

    fn walk_node_for_security(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let node_kind = node.kind();

        // zmodload check
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "zmodload" {
                    return Err("Dangerous pattern detected: zsh dangerous command/expansion (zmodload)".to_string());
                }
            }
        }

        // process substitution <() or >()
        if node_kind == "process_substitution" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("<(") {
                return Err("Dangerous pattern detected: process substitution".to_string());
            }
            if text.starts_with(">(") {
                return Err("Dangerous pattern detected: process substitution".to_string());
            }
        }

        // legacy expansions $[] (not strictly supported in all bash, often handled as expansion)
        // Check for node type "expansion" or similar, and check text.
        // In tree-sitter-bash it might be `expansion` or `arithmetic_expansion`
        if node_kind == "expansion" || node_kind == "arithmetic_expansion" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("$[") && text.ends_with("]") {
                return Err("Dangerous pattern detected: dynamic command names (subshells/expansions) are not allowed".to_string());
            }
        }

        // Just in case it's not parsed properly, check string content for $[] as fallback, but only if it's text.
        // But doing it robustly:
        if node_kind == "word" || node_kind == "raw_string" {
             let text = &source[node.start_byte()..node.end_byte()];
             if text.starts_with("$[") && text.ends_with("]") {
                  return Err("Dangerous pattern detected: dynamic command names (subshells/expansions) are not allowed".to_string());
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
