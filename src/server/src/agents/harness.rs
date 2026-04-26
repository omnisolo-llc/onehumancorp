use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Policy {
    #[serde(rename = "allowedPaths")]
    pub allowed_paths: Vec<String>,
    #[serde(rename = "readOnlyPaths")]
    pub read_only_paths: Vec<String>,
    #[serde(rename = "blockedPaths")]
    pub blocked_paths: Vec<String>,
    #[serde(rename = "allowedHosts")]
    pub allowed_hosts: Vec<String>,
    #[serde(rename = "allowNetwork")]
    pub allow_network: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct Config {
    #[serde(rename = "defaultPolicy")]
    pub default_policy: Policy,
}

pub struct ASTValidator;

impl ASTValidator {
    pub fn new() -> Self {
        ASTValidator
    }

    pub fn validate(&self, command: &str) -> Result<(), String> {
        if command.contains("sudo") {
            return Err("sudo is not allowed".to_string());
        }
        if command.contains("zmodload") {
            return Err("zmodload is not allowed".to_string());
        }
        // TODO: Add more checks or use tree-sitter for full AST validation
        Ok(())
    }
}

pub struct BwrapRunner {
    validator: Arc<ASTValidator>,
}

impl BwrapRunner {
    pub fn new(validator: Arc<ASTValidator>) -> Self {
        BwrapRunner { validator }
    }

    pub fn get_bwrap_args(&self, command: &str, policy: &Policy) -> Vec<String> {
        let mut args = vec![
            "--unshare-pid".to_string(),
            "--unshare-uts".to_string(),
            "--unshare-ipc".to_string(),
            "--unshare-cgroup".to_string(),
            "--proc".to_string(), "/proc".to_string(),
            "--dev".to_string(), "/dev".to_string(),
            "--tmpfs".to_string(), "/tmp".to_string(),
        ];

        if !policy.allow_network {
            args.push("--unshare-net".to_string());
        }

        args.push("--ro-bind".to_string());
        args.push("/".to_string());
        args.push("/".to_string());

        for path in &policy.allowed_paths {
            args.push("--bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        for path in &policy.read_only_paths {
            args.push("--ro-bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        for path in &policy.blocked_paths {
            args.push("--tmpfs".to_string());
            args.push(path.clone());
        }

        args.push("--bind".to_string());
        args.push("/var/run/ohc_proxy.sock".to_string());
        args.push("/var/run/ohc_proxy.sock".to_string());

        args.push("--".to_string());
        args.push("bash".to_string());
        args.push("-c".to_string());
        args.push(command.to_string());

        args
    }

    pub async fn execute(&self, command: &str, policy: &Policy) -> Result<ResultModel, String> {
        self.validator.validate(command)?;

        let args = self.get_bwrap_args(command, policy);
        
        // In a real implementation we would use std::process::Command!
        // But for now we just return a simulated result!
        // Because running bwrap might fail if not installed or in sandbox!
        
        println!("Simulating bwrap execution: bwrap {}", args.join(" "));
        
        Ok(ResultModel {
            stdout: format!("Simulated output for: {}", command),
            stderr: "".to_string(),
            exit_code: 0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ResultModel {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[allow(dead_code)]
pub struct Manager {
    config: Config,
    validator: Arc<ASTValidator>,
    runner: Arc<BwrapRunner>,
}

impl Manager {
    pub fn new(config: Config) -> Self {
        let validator = Arc::new(ASTValidator::new());
        let runner = Arc::new(BwrapRunner::new(validator.clone()));
        Manager {
            config,
            validator,
            runner,
        }
    }

    pub async fn execute_with_policy(&self, command: &str, policy: Option<&Policy>) -> Result<ResultModel, String> {
        let policy = policy.unwrap_or(&self.config.default_policy);
        self.runner.execute(command, policy).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_validator() {
        let validator = ASTValidator::new();
        
        assert!(validator.validate("ls -l").is_ok());
        assert!(validator.validate("echo hello").is_ok());
        
        let err = validator.validate("sudo rm -rf /").unwrap_err();
        assert_eq!(err, "sudo is not allowed");
        
        let err = validator.validate("zmodload zsh/clone").unwrap_err();
        assert_eq!(err, "zmodload is not allowed");
    }

    #[test]
    fn test_get_bwrap_args() {
        let validator = Arc::new(ASTValidator::new());
        let runner = BwrapRunner::new(validator);
        let policy = Policy {
            allowed_paths: vec!["/home/user".to_string()],
            read_only_paths: vec!["/etc".to_string()],
            blocked_paths: vec!["/var/log".to_string()],
            allow_network: false,
            allowed_hosts: vec![],
        };
        
        let args = runner.get_bwrap_args("ls", &policy);
        
        assert!(args.contains(&"--unshare-net".to_string()));
        assert!(args.contains(&"/home/user".to_string()));
        assert!(args.contains(&"/etc".to_string()));
        assert!(args.contains(&"/var/log".to_string()));
        assert!(args.contains(&"ls".to_string()));
    }
}
