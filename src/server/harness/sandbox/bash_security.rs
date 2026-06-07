use tree_sitter::{Node, Parser};

pub struct ParsedCommand {
    parser: Parser,
}

impl ParsedCommand {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_bash::LANGUAGE.into())
            .expect("Error loading bash grammar");
        Self { parser }
    }

    pub fn parse_for_security(&mut self, cmd: &str) -> Result<(), String> {
        self.validate_carriage_return(cmd)?;

        let tree = self
            .parser
            .parse(cmd, None)
            .ok_or("Failed to parse command")?;
        let root_node = tree.root_node();

        self.walk_node_for_security(root_node, cmd)
    }

    fn validate_carriage_return(&self, cmd: &str) -> Result<(), String> {
        if cmd.contains("\r\n") || cmd.contains('\r') {
            return Err("Dangerous pattern detected: carriage return \\r\\n".to_string());
        }
        Ok(())
    }

    fn walk_node_for_security(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let node_kind = node.kind();

        // 1. ValidateZshDangerousCommands
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "zmodload" {
                    return Err("Dangerous pattern detected: zmodload".to_string());
                }
            }
        }

        // Catch =curl or similar zsh expansions as command arguments
        if node_kind == "word" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("=curl") || text.starts_with("=wget") {
                return Err("Dangerous pattern detected: zsh equals expansion".to_string());
            }
        }

        // 2. ValidateProcessSubstitution (from earlier AST parser)
        if node_kind == "process_substitution" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("<(") {
                return Err("Dangerous pattern detected: <() process substitution".to_string());
            }
            if text.starts_with(">(") {
                return Err("Dangerous pattern detected: >() process substitution".to_string());
            }
        }

        // 3. ValidateIFSInjection & DangerousVariables in variable_assignment
        if node_kind == "variable_assignment" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let var_name = &source[name_node.start_byte()..name_node.end_byte()];
                if var_name == "IFS" {
                    return Err("Dangerous pattern detected: IFS injection".to_string());
                }
                if var_name == "LD_PRELOAD"
                    || var_name == "PROMPT_COMMAND"
                    || var_name == "LD_LIBRARY_PATH"
                {
                    return Err(format!(
                        "Dangerous pattern detected: dangerous variable assignment {}",
                        var_name
                    ));
                }
            }
        }

        // Catch usage of dangerous variables
        if node_kind == "variable_name" {
            let var_name = &source[node.start_byte()..node.end_byte()];
            if var_name == "LD_PRELOAD"
                || var_name == "PROMPT_COMMAND"
                || var_name == "LD_LIBRARY_PATH"
            {
                return Err(format!(
                    "Dangerous pattern detected: dangerous variable usage {}",
                    var_name
                ));
            }
        }

        // 4. ValidateJqCommand
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "jq" {
                    // Check arguments for jq
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        let child_text = &source[child.start_byte()..child.end_byte()];
                        if child_text.contains("@sh")
                            || child_text.contains("env")
                            || child_text.contains("input")
                        {
                            return Err("Dangerous pattern detected: unsafe jq command".to_string());
                        }
                    }
                }
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
        let mut parser = ParsedCommand::new();
        assert!(parser.parse_for_security("echo 'hello world'").is_ok());
        assert!(parser.parse_for_security("ls -l /tmp").is_ok());
        assert!(parser.parse_for_security("cat file.txt | grep foo").is_ok());
    }

    #[test]
    fn test_block_zmodload() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("zmodload zsh/net/tcp");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: zmodload");
    }

    #[test]
    fn test_block_zsh_equals() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("ls =curl");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: zsh equals expansion"
        );
    }

    #[test]
    fn test_block_process_substitution_out() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("echo 'test' > >(cat)");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: >() process substitution"
        );
    }

    #[test]
    fn test_block_process_substitution_in() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("cat < <(echo 'test')");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: <() process substitution"
        );
    }

    #[test]
    fn test_block_legacy_expansion() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("echo $[1+1]");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: $[] legacy expansion"
        );
    }

    #[test]
    fn test_block_carriage_return() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("echo 'test'\r\n");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: carriage return \\r\\n"
        );
    }

    #[test]
    fn test_block_ifs_injection() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("IFS=; echo test");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: IFS injection"
        );
    }

    #[test]
    fn test_block_dangerous_variables_assignment() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("LD_PRELOAD=/usr/lib/evil.so ls");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: dangerous variable assignment LD_PRELOAD"
        );
    }

    #[test]
    fn test_block_dangerous_variables_usage() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("echo $PROMPT_COMMAND");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: dangerous variable usage PROMPT_COMMAND"
        );
    }

    #[test]
    fn test_block_jq_command() {
        let mut parser = ParsedCommand::new();
        let res = parser.parse_for_security("jq -r '. | @sh' data.json");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "Dangerous pattern detected: unsafe jq command"
        );
    }
}
