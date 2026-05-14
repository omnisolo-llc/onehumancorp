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

        // legacy expansions $[] (not strictly supported in all bash, often handled as expansion)
        if node_kind == "expansion" || node_kind == "arithmetic_expansion" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("$[") && text.ends_with("]") {
                return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
            }
        }

        if node_kind == "word" || node_kind == "raw_string" {
             let text = &source[node.start_byte()..node.end_byte()];
             if text.starts_with("$[") && text.ends_with("]") {
                  return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
             }
        }

        // Check for =command zsh dangerous expansions ONLY if it matches exactly `=command` form
        // without quotes or spaces, typically identified as a word.
        if node_kind == "word" {
             let text = &source[node.start_byte()..node.end_byte()];
             if text.starts_with("=") && text.len() > 1 && !text.contains(" ") {
                  return Err("Dangerous pattern detected: = zsh expansion".to_string());
             }
        }

        // Check for carriage return \r\n
        if node_kind == "word" || node_kind == "raw_string" {
             let text = &source[node.start_byte()..node.end_byte()];
             if text.contains("\r\n") {
                  return Err("Dangerous pattern detected: \\r\\n carriage return".to_string());
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
        // allow "=" inside quotes or as a standalone arg
        assert!(parser.parse_for_security("echo \"=\"").is_ok());
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
        let res = parser.parse_for_security("echo $[1+1]");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: $[] legacy expansion");
    }

    #[test]
    fn test_block_zsh_expansion() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("=curl");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: = zsh expansion");
    }
}
