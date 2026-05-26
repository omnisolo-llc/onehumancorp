use regex::Regex;

pub fn validate_bash_command(command: &str) -> Result<(), String> {
    let exact_patterns = vec![
        ">(",
        "<(",
        "$[",
    ];

    for pattern in exact_patterns {
        if command.contains(pattern) {
            return Err(format!(
                "Security violation: dangerous pattern '{}' detected in bash command",
                pattern
            ));
        }
    }

    // Use word boundaries for commands to avoid matching parts of file names
    let word_patterns = vec![
        "zmodload",
        "eval",
        "exec",
    ];

    for pattern in word_patterns {
        let re = Regex::new(&format!(r"\b{}\b", pattern)).unwrap();
        if re.is_match(command) {
            return Err(format!(
                "Security violation: dangerous pattern '{}' detected in bash command",
                pattern
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_bash_command_safe() {
        let cmd = "ls -la /tmp";
        assert!(validate_bash_command(cmd).is_ok());

        let cmd2 = "grep 'hello' file.txt | sort";
        assert!(validate_bash_command(cmd2).is_ok());

        let cmd3 = "cat evaluation_results.txt";
        assert!(validate_bash_command(cmd3).is_ok());

        let cmd4 = "grep -r 'executing' .";
        assert!(validate_bash_command(cmd4).is_ok());
    }

    #[test]
    fn test_validate_bash_command_zmodload() {
        let cmd = "zmodload zsh/net/tcp";
        let res = validate_bash_command(cmd);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("zmodload"));
    }

    #[test]
    fn test_validate_bash_command_process_substitution() {
        let cmd = "cat <(ls -l)";
        let res = validate_bash_command(cmd);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("<("));

        let cmd2 = "diff <(ls -l) <(ls -al)";
        let res2 = validate_bash_command(cmd2);
        assert!(res2.is_err());
        assert!(res2.unwrap_err().contains("<("));

        let cmd3 = "tee >(grep abc)";
        let res3 = validate_bash_command(cmd3);
        assert!(res3.is_err());
        assert!(res3.unwrap_err().contains(">("));
    }

    #[test]
    fn test_validate_bash_command_legacy_expansion() {
        let cmd = "echo $[1+1]";
        let res = validate_bash_command(cmd);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("$["));
    }

    #[test]
    fn test_validate_bash_command_eval() {
        let cmd = "eval 'echo hello'";
        let res = validate_bash_command(cmd);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("eval"));
    }

    #[test]
    fn test_validate_bash_command_exec() {
        let cmd = "exec /bin/sh";
        let res = validate_bash_command(cmd);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("exec"));
    }
}
