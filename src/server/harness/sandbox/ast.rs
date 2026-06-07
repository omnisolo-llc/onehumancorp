use tree_sitter::{Node, Parser};
use server_telemetry::record_harness_security_divergence;

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
        let (sanitized_cmd, _heredocs) = self.extract_heredocs(cmd);

        if let Err(e) = self.validate_carriage_return(&sanitized_cmd) {
            record_harness_security_divergence("carriage_return", &sanitized_cmd);
            return Err(e);
        }

        let tree = self.parser.parse(&sanitized_cmd, None).ok_or("Failed to parse command")?;
        let root_node = tree.root_node();

        if let Err(e) = self.walk_node_for_security(root_node, &sanitized_cmd) {
            record_harness_security_divergence(&e, &sanitized_cmd);
            return Err(e);
        }
        Ok(())
    }

    /// Replaces heredocs with placeholders so their content doesn't trigger false positives.
    fn extract_heredocs(&self, cmd: &str) -> (String, Vec<String>) {
        // A simple regex approach to find `<< 'EOF'` and replace until `EOF`
        // Rust's regex crate does not support backreferences like `\1`.
        // We will implement a simple manual parser to extract heredocs with quoted delimiters.

        let mut heredocs = Vec::new();
        let mut sanitized = cmd.to_string();

        // We look for << 'EOF' or << "EOF"
        let re_start = regex::Regex::new(r#"<<\s*(['"])([^'"]+)['"]"#).unwrap();

        loop {
            let mat = match re_start.find(&sanitized) {
                Some(m) => m,
                None => break,
            };

            let match_str = &sanitized[mat.start()..mat.end()];
            let caps = match re_start.captures(match_str) {
                Some(c) => c,
                None => break,
            };

            let quote = caps.get(1).unwrap().as_str();
            let delimiter = caps.get(2).unwrap().as_str();

            let start_idx = mat.start();
            let search_start = mat.end();

            // Re-verify that quote matches the end quote since we couldn't use backreferences
            let end_char = match_str.chars().last().unwrap();
            if quote.chars().next().unwrap() != end_char {
                // If it doesn't match properly, avoid infinite loop
                break;
            }

            // Look for the delimiter on a line by itself
            let end_pattern = format!("\n{}$", delimiter);
            let end_pattern_nl = format!("\n{}\n", delimiter);

            let end_idx;
            let match_len;

            if let Some(pos) = sanitized[search_start..].find(&end_pattern_nl) {
                end_idx = search_start + pos;
                match_len = end_pattern_nl.len();
            } else if let Some(pos) = sanitized[search_start..].find(&end_pattern) {
                end_idx = search_start + pos;
                match_len = end_pattern.len();
            } else {
                // Malformed or no end found, break to avoid infinite loop
                break;
            }

            let full_match = &sanitized[start_idx..end_idx + match_len];
            heredocs.push(full_match.to_string());

            // Replace with a masked heredoc, but change the delimiter so we don't match it again
            let replacement = format!("<< MASKED_{}\n_MASKED_\nMASKED_{}", delimiter, delimiter);
            sanitized.replace_range(start_idx..end_idx + match_len, &replacement);
        }

        (sanitized, heredocs)
    }

    fn validate_carriage_return(&self, cmd: &str) -> Result<(), String> {
        if cmd.contains('\r') {
            return Err("Dangerous pattern detected: Carriage return \\r injection".to_string());
        }
        Ok(())
    }

    fn walk_node_for_security(&self, node: Node<'_>, source: &str) -> Result<(), String> {
        let node_kind = node.kind();

<<<<<<< HEAD
        // command checks
        if node_kind == "command" {
            if let Some(command_name_node) = node.child_by_field_name("name") {
=======
        // zmodload check
        if node_kind == "command" && let Some(command_name_node) = node.child_by_field_name("name") {
>>>>>>> 87851578 (chore: fix workspace compilation and clippy errors)
                let name = &source[command_name_node.start_byte()..command_name_node.end_byte()];
                if name == "zmodload" {
                    return Err("Dangerous pattern detected: zmodload".to_string());
                }
<<<<<<< HEAD

                // Block `=curl` (zsh expansion to path) or similar dangerous zsh commands
                if name.starts_with('=') {
                    return Err(format!("Dangerous pattern detected: Zsh equals expansion {}", name));
                }

                // Jq Validation (e.g., prevent reading env or running arbitrary code if args are unquoted)
                // For simplicity, we just flag jq if we detect dangerous flags or env reading.
                if name == "jq" {
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        // tree-sitter-bash parses single-quoted strings as `raw_string` or similar
                        // and double-quoted as `string`
                        let kind = child.kind();
                        if kind == "word" || kind == "raw_string" || kind == "string" {
                            let arg = &source[child.start_byte()..child.end_byte()];
                            if arg.contains("env") || arg.contains("system") {
                                return Err("Dangerous pattern detected: jq with env/system access".to_string());
                            }
                        }
                    }
                }
            }
=======
>>>>>>> 87851578 (chore: fix workspace compilation and clippy errors)
        }

        // Dangerous Variables Check
        if node_kind == "variable_assignment" {
            if let Some(var_name_node) = node.child_by_field_name("name") {
                let var_name = &source[var_name_node.start_byte()..var_name_node.end_byte()];
                let dangerous_vars = ["LD_PRELOAD", "LD_LIBRARY_PATH", "PROMPT_COMMAND", "BASH_ENV", "ENV"];
                if dangerous_vars.contains(&var_name) {
                    return Err(format!("Dangerous pattern detected: Setting dangerous variable {}", var_name));
                }

                // Validate IFS Injection
                if var_name == "IFS" {
                    // Check if value is being manipulated maliciously
                    if let Some(value_node) = node.child_by_field_name("value") {
                        let val = &source[value_node.start_byte()..value_node.end_byte()];
                        if val.contains("$(") || val.contains("`") {
                            return Err("Dangerous pattern detected: IFS injection".to_string());
                        }
                    } else {
                        // Bare IFS assignment is often suspicious
                        return Err("Dangerous pattern detected: IFS manipulation".to_string());
                    }
                }
            }
        }

        // Validate IFS Injection in expansion
        if node_kind == "expansion" || node_kind == "simple_expansion" {
             let text = &source[node.start_byte()..node.end_byte()];
             if text.contains("IFS") {
                 // Check context, but flag for now if it's purely $IFS manipulation in a weird way
                 // A basic check: we allow $IFS, but we flag ${IFS=...} or similar
                 if text.starts_with("${IFS=") || text.starts_with("${IFS:=") || text.starts_with("${IFS+") {
                     return Err("Dangerous pattern detected: IFS manipulation in expansion".to_string());
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

<<<<<<< HEAD
#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_extract_heredocs() {
        let parser = ASTParser::new();
        let cmd = "cat << 'EOF'\nmalicious <()\nEOF\necho 'done'";
        let (sanitized, heredocs) = parser.extract_heredocs(cmd);
        assert_eq!(heredocs.len(), 1);
        assert!(heredocs[0].contains("malicious"));
        assert!(!sanitized.contains("malicious"));
        assert!(sanitized.contains("MASKED"));
        assert!(sanitized.contains("echo 'done'"));
    }

    #[test]
    fn test_validate_carriage_return() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("echo 'hello\rworld'");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: Carriage return \\r injection");
    }

    #[test]
    fn test_jq_validation() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("jq 'env'");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: jq with env/system access");
    }

    #[test]
    fn test_zsh_equals_expansion() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("=curl http://evil.com");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: Zsh equals expansion =curl");
    }

    #[test]
    fn test_dangerous_variables() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("LD_PRELOAD=/tmp/evil.so echo 'hello'");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: Setting dangerous variable LD_PRELOAD");
    }

    #[test]
    fn test_ifs_injection() {
        let mut parser = ASTParser::new();
        let res = parser.parse_for_security("IFS=$(echo '\n') cat /etc/passwd");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Dangerous pattern detected: IFS injection");

        let res2 = parser.parse_for_security("echo ${IFS=;}");
        assert!(res2.is_err());
        assert_eq!(res2.unwrap_err(), "Dangerous pattern detected: IFS manipulation in expansion");
    }

    #[test]
    fn test_coverage_edge_cases() {
        let mut parser = ASTParser::new();
        // heredoc with normal text
        assert!(parser.parse_for_security("cat << \"EOF\"\nhello\nEOF").is_ok());
        // malformed heredoc
        assert!(parser.parse_for_security("cat << 'EOF'\nunterminated heredoc").is_ok());

        // IFS naked assignment
        assert!(parser.parse_for_security("IFS= cat /etc/passwd").is_err());

        // legitimate jq
        assert!(parser.parse_for_security("jq '.' file.json").is_ok());

        // dangerous jq via raw string
        assert!(parser.parse_for_security("jq 'env' file.json").is_err());

        // dangerous jq via double string
        assert!(parser.parse_for_security("jq \"env\" file.json").is_err());
=======
impl Default for ASTParser {
    fn default() -> Self {
        Self::new()
>>>>>>> 87851578 (chore: fix workspace compilation and clippy errors)
    }
}
