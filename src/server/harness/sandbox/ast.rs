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
        let res = self.parse_for_security_inner(cmd);
        if let Err(ref e) = res {
            ::server_telemetry::record_sandbox_violation(e);
        }
        res
    }

    fn parse_for_security_inner(&mut self, cmd: &str) -> Result<(), String> {
        let tree = self.parser.parse(cmd, None).ok_or("Failed to parse command")?;
        let root_node = tree.root_node();

        self.walk_node_for_security(root_node, cmd)
    }

    fn walk_node_for_security(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let node_kind = node.kind();

        // zmodload check
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "zmodload" {
                    return Err("Dangerous pattern detected: zmodload".to_string());
                }
            }
        }

        // process substitution <() or >()
        if node_kind == "process_substitution" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("<(") {
                return Err("Dangerous pattern detected: <() process substitution".to_string());
            }
            if text.starts_with(">(") {
                return Err("Dangerous pattern detected: >() process substitution".to_string());
            }
        }

        // command substitution e.g. $(...) or `...`
        if node_kind == "command_substitution" {
            return Err("Dangerous pattern detected: subshell execution".to_string());
        }

        // redirect statements
        if node_kind == "file_redirect" {
            // Check if destination is process substitution first
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "process_substitution" {
                    let text = &source[child.start_byte()..child.end_byte()];
                    if text.starts_with("<(") {
                        return Err("Dangerous pattern detected: <() process substitution".to_string());
                    }
                    if text.starts_with(">(") {
                        return Err("Dangerous pattern detected: >() process substitution".to_string());
                    }
                }
            }
            return Err("Dangerous pattern detected: file redirection".to_string());
        }

        // legacy expansions $[] (not strictly supported in all bash, often handled as expansion)
        // Check for node type "expansion" or similar, and check text.
        // In tree-sitter-bash it might be `expansion` or `arithmetic_expansion`
        if node_kind == "expansion" || node_kind == "arithmetic_expansion" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("$[") && text.ends_with("]") {
                return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
            }
        }

        // Just in case it's not parsed properly, check string content for $[] as fallback, but only if it's text.
        // But doing it robustly:
        if node_kind == "word" || node_kind == "raw_string" {
             let text = &source[node.start_byte()..node.end_byte()];
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
        assert!(parser.parse_for_security_inner("echo 'hello world'").is_ok());
        assert!(parser.parse_for_security_inner("ls -l /tmp").is_ok());
        assert!(parser.parse_for_security_inner("cat file.txt | grep foo").is_ok());
    }

    #[test]
    fn test_block_zmodload() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security_inner("zmodload zsh/net/tcp");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: zmodload");
    }

    #[test]
    fn test_block_subshell() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security_inner("echo \"su\"$(echo \"do\")");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: subshell execution");

        let res2 = parser.parse_for_security_inner("echo `ls`");
        assert!(res2.is_err());
        assert_eq!(res2.unwrap_err(), "Dangerous pattern detected: subshell execution");
    }

    #[test]
    fn test_block_redirection() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security_inner("echo test > /etc/passwd");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: file redirection");
    }

    #[test]
    fn test_block_process_substitution_out() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security_inner("echo 'test' > >(cat)");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[test]
    fn test_block_process_substitution_in() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security_inner("cat < <(echo 'test')");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: <() process substitution");
    }

    #[test]
    fn test_block_legacy_expansion() {
        let mut parser = ASTParser::new();
        // $[] might be parsed as word or expansion depending on tree-sitter-bash grammar rules
        let res = parser.parse_for_security_inner("echo $[1+1]");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: $[] legacy expansion");
    }
}
