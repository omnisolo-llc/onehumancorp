use tree_sitter::{Node, Parser};

pub struct ParsedCommand {
    pub cmd: String,
    parser: Parser,
}

impl ParsedCommand {
    pub fn new(cmd: &str) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_bash::LANGUAGE.into())
            .expect("Error loading bash grammar");
        Self {
            cmd: cmd.to_string(),
            parser,
        }
    }

    pub fn parse_and_validate(&mut self) -> Result<(), String> {
        let tree = self.parser.parse(&self.cmd, None).ok_or("Failed to parse command")?;
        let root_node = tree.root_node();

        self.validate_carriage_return()?;
        self.validate_ifs_injection()?;

        self.walk_node_for_security(root_node)
    }

    fn walk_node_for_security(&self, node: Node<'_>) -> Result<(), String> {
        let node_kind = node.kind();
        let source = &self.cmd;
        let text = &source[node.start_byte()..node.end_byte()];

        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                self.validate_zsh_dangerous_commands(name)?;
                self.validate_jq_command(name, node, source)?;

                if name.contains("$(") || name.contains("`") || name.contains("${") || name.contains("$[") {
                    return Err("Dangerous pattern detected: dynamic command names (subshells/expansions) are not allowed".to_string());
                }

                let mut cursor = command_name_node.walk();
                for child in command_name_node.children(&mut cursor) {
                    let child_kind = child.kind();
                    if child_kind == "command_substitution" || child_kind == "expansion" || child_kind == "arithmetic_expansion" {
                        return Err("Dangerous pattern detected: dynamic command names (subshells/expansions) are not allowed".to_string());
                    }
                }
            }
        }

        if node_kind == "variable_name" || node_kind == "variable_assignment" {
             self.validate_dangerous_variables(node, source)?;
        }

        if node_kind == "process_substitution" {
            if text.starts_with("<(") || text.starts_with(">(") {
                return Err("Dangerous pattern detected: process substitution".to_string());
            }
        }

        // legacy $[]
        if node_kind == "expansion" || node_kind == "arithmetic_expansion" || node_kind == "word" || node_kind == "raw_string" {
            if text.starts_with("$[") && text.ends_with("]") {
                return Err("Dangerous pattern detected: dynamic command names (subshells/expansions) are not allowed".to_string());
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_node_for_security(child)?;
        }

        Ok(())
    }

    fn validate_jq_command(&self, name: &str, node: Node<'_>, source: &str) -> Result<(), String> {
        if name == "jq" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                let text = &source[child.start_byte()..child.end_byte()];
                // Block explicit env usage in jq filters
                if text == "env" || text.starts_with("env.") {
                    return Err("Dangerous pattern detected: jq env access".to_string());
                }
            }
        }
        Ok(())
    }

    fn validate_dangerous_variables(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let text = &source[node.start_byte()..node.end_byte()];
        if text.contains("LD_PRELOAD") || text.contains("PROMPT_COMMAND") || text.contains("BASH_ENV") {
            return Err("Dangerous pattern detected: dangerous variable manipulation".to_string());
        }
        Ok(())
    }

    fn validate_zsh_dangerous_commands(&self, name: &str) -> Result<(), String> {
        if name == "zmodload" || name.starts_with("=") {
            // `=` is often a zsh equals expansion (e.g., `=curl`)
            return Err(format!("Dangerous pattern detected: zsh dangerous command/expansion ({})", name));
        }
        Ok(())
    }

    fn validate_carriage_return(&self) -> Result<(), String> {
        if self.cmd.contains("\r") {
            return Err("Dangerous pattern detected: carriage return".to_string());
        }
        Ok(())
    }

    fn validate_ifs_injection(&self) -> Result<(), String> {
        if self.cmd.contains("IFS=") || self.cmd.contains("IFS =") {
            return Err("Dangerous pattern detected: IFS injection".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carriage_return() {
        let mut cmd = ParsedCommand::new("echo hello\r\n");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: carriage return");
    }

    #[test]
    fn test_ifs_injection() {
        let mut cmd = ParsedCommand::new("IFS=; echo hello");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: IFS injection");
    }

    #[test]
    fn test_zmodload() {
        let mut cmd = ParsedCommand::new("zmodload zsh/net/tcp");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: zsh dangerous command/expansion (zmodload)");
    }

    #[test]
    fn test_zsh_equals() {
        let mut cmd = ParsedCommand::new("=curl http://example.com");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: zsh dangerous command/expansion (=curl)");
    }

    #[test]
    fn test_dangerous_variables() {
        let mut cmd = ParsedCommand::new("LD_PRELOAD=/tmp/evil.so ls");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: dangerous variable manipulation");
    }

    #[test]
    fn test_jq_env() {
        let mut cmd = ParsedCommand::new("jq 'env' file.json");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: jq env access");
    }

    #[test]
    fn test_jq_valid() {
        let mut cmd = ParsedCommand::new("jq '.' file.json");
        assert!(cmd.parse_and_validate().is_ok());
    }

    #[test]
    fn test_process_substitution() {
        let mut cmd = ParsedCommand::new("cat <(ls)");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: process substitution");
    }

    #[test]
    fn test_command_substitution() {
        let mut cmd = ParsedCommand::new("$(ls)");
        assert_eq!(cmd.parse_and_validate().unwrap_err(), "Dangerous pattern detected: dynamic command names (subshells/expansions) are not allowed");
    }

    #[test]
    fn test_valid_variable_expansion() {
        let mut cmd = ParsedCommand::new("echo ${VAR}");
        assert!(cmd.parse_and_validate().is_ok());
    }
}
