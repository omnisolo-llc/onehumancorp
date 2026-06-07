use tree_sitter::{Node, Parser};
use server_telemetry::record_harness_security_divergence;

pub struct ParsedCommand<'a> {
    pub source: &'a str,
}

impl<'a> ParsedCommand<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn parse(&self) -> Result<(), String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_bash::LANGUAGE.into())
            .expect("Error loading bash grammar");

        let tree = parser.parse(self.source, None).ok_or("Failed to parse command")?;
        let root_node = tree.root_node();

        // Check if there's divergence between simple regex and AST
        let has_carriage_return = self.source.contains('\r');
        if has_carriage_return {
             record_harness_security_divergence();
             return Err("Dangerous pattern detected: \\r\\n carriage return".to_string());
        }

        self.walk_node_for_security(root_node, self.source)
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

        // Zsh equals expansion, e.g. =curl
        if node_kind == "word" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with('=') && text.len() > 1 {
                return Err(format!("Dangerous pattern detected: Zsh equals expansion {}", text));
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

        // command substitution $()
        if node_kind == "command_substitution" {
            let text = &source[node.start_byte()..node.end_byte()];
            if text.starts_with("$(") {
                return Err("Dangerous pattern detected: $() command substitution".to_string());
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

        // IFS injection check
        if node_kind == "variable_assignment" {
             let text = &source[node.start_byte()..node.end_byte()];
             if text.starts_with("IFS=") {
                 return Err("Dangerous pattern detected: IFS injection".to_string());
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
        let cmd = ParsedCommand::new("echo 'hello world'");
        assert!(cmd.parse().is_ok());
        let cmd2 = ParsedCommand::new("ls -l /tmp");
        assert!(cmd2.parse().is_ok());
        let cmd3 = ParsedCommand::new("cat file.txt | grep foo");
        assert!(cmd3.parse().is_ok());
    }

    #[test]
    fn test_block_carriage_return() {
        let cmd = ParsedCommand::new("echo 'hello\rworld'");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: \\r\\n carriage return");
    }

    #[test]
    fn test_block_zmodload() {
        let cmd = ParsedCommand::new("zmodload zsh/net/tcp");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: zmodload");
    }

    #[test]
    fn test_block_zsh_equals_expansion() {
        let cmd = ParsedCommand::new("echo =curl");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: Zsh equals expansion =curl");
    }

    #[test]
    fn test_block_process_substitution_out() {
        let cmd = ParsedCommand::new("echo 'test' > >(cat)");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[test]
    fn test_block_process_substitution_in() {
        let cmd = ParsedCommand::new("cat < <(echo 'test')");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: <() process substitution");
    }

    #[test]
    fn test_block_command_substitution() {
        let cmd = ParsedCommand::new("echo $(ls)");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: $() command substitution");
    }

    #[test]
    fn test_block_legacy_expansion() {
        let cmd = ParsedCommand::new("echo $[1+1]");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: $[] legacy expansion");
    }

    #[test]
    fn test_block_ifs_injection() {
        let cmd = ParsedCommand::new("IFS=; echo hello");
        let res = cmd.parse();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: IFS injection");
    }
}
