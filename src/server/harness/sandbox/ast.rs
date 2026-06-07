use tree_sitter::{Node, Parser};
use regex::Regex;

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

    pub fn extract_heredocs(cmd: &str) -> String {
        // Safe heredoc extraction
        if let Ok(re) = Regex::new(r"(?sm)<<-?\s*(?P<delim>\w+)[\r\n].*?^(?P=delim)$") {
            return re.replace_all(cmd, "").to_string();
        }
        cmd.to_string()
    }

    pub fn parse_for_security(&mut self, cmd: &str) -> Result<(), String> {
        if cmd.contains('\r') {
            return Err("Dangerous pattern detected: CRLF injection".to_string());
        }

        let cmd_to_parse = Self::extract_heredocs(cmd);

        let tree = self.parser.parse(&cmd_to_parse, None).ok_or("Failed to parse command")?;
        let root_node = tree.root_node();

        self.walk_node_for_security(root_node, &cmd_to_parse)
    }

    fn walk_node_for_security(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let node_kind = node.kind();

        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "zmodload" {
                    return Err("Dangerous pattern detected: zmodload".to_string());
                }
                if name.starts_with('=') {
                    return Err("Dangerous pattern detected: zsh equals expansion".to_string());
                }

                // ValidateJqCommand
                if name == "jq" {
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        let arg_text = &source[child.start_byte()..child.end_byte()];
                        if arg_text.contains("env") || arg_text.contains("system") {
                            return Err("Dangerous pattern detected: unsafe jq command".to_string());
                        }
                    }
                }
            }
        }

        // ValidateDangerousVariables & IFSInjection
        if node_kind == "variable_assignment" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = &source[name_node.start_byte()..name_node.end_byte()];
                if name == "IFS" {
                    return Err("Dangerous pattern detected: IFS injection".to_string());
                }
                if name == "LD_PRELOAD" || name == "PROMPT_COMMAND" {
                    return Err(format!("Dangerous pattern detected: unsafe variable assignment {}", name));
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

        // command substitution $() or ``
        if node_kind == "command_substitution" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("`") {
                 return Err("Dangerous pattern detected: command substitution".to_string());
            }
            if text.starts_with("$(") {
                 return Err("Dangerous pattern detected: command substitution".to_string());
            }
        }

        // legacy expansions $[]
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
    fn test_block_zsh_equals() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("=curl http://evil.com");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: zsh equals expansion");
    }

    #[test]
    fn test_block_crlf() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo hello\r\nrm -rf /");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: CRLF injection");
    }

    #[test]
    fn test_block_ifs_injection() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("IFS=; echo hello");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: IFS injection");
    }

    #[test]
    fn test_block_dangerous_variables() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("LD_PRELOAD=/evil.so ls");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: unsafe variable assignment LD_PRELOAD");
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
    fn test_block_command_substitution() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo $(whoami)");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: command substitution");
    }

    #[test]
    fn test_block_legacy_expansion() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo $[1+1]");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: $[] legacy expansion");
    }
}
