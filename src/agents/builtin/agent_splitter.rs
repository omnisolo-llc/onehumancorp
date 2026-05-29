use std::collections::HashSet;

use crate::tools::Tool;

/// Decision for architectural execution mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplitDecision {
    SingleAgent,
    MultiAgent,
}

/// SOTA Harness Patterns:
/// 1. Single-agent vs Multi-agent: Maximize single-agent first.
/// Mechanic: Split into multi-agent ONLY when overlapping tools exceed ~10
/// or clear domain separation exists.
pub struct AgentSplitter;

impl AgentSplitter {
    /// Extracts a presumed domain from a tool's name or description.
    /// This is a simple heuristic mapping.
    fn extract_domain(tool: &Tool) -> String {
        let name_lower = tool.name.to_lowercase();
        let desc_lower = tool.description.to_lowercase();
        let combined = format!("{} {}", name_lower, desc_lower);

        if combined.contains("git") || combined.contains("repo") {
            return "version_control".to_string();
        }
        if combined.contains("file") || combined.contains("read") || combined.contains("write") || combined.contains("grep") || combined.contains("bash") {
            return "filesystem".to_string();
        }
        if combined.contains("http") || combined.contains("fetch") || combined.contains("web") {
            return "network".to_string();
        }
        if combined.contains("sql") || combined.contains("db") || combined.contains("database") {
            return "database".to_string();
        }

        // Fallback domain
        "general".to_string()
    }

    pub fn decide_split(tools: &[Tool]) -> SplitDecision {
        // Mechanic: Split into multi-agent ONLY when overlapping tools exceed ~10
        if tools.len() > 10 {
            return SplitDecision::MultiAgent;
        }

        // Check for clear domain separation.
        let mut domains = HashSet::new();
        for tool in tools {
            domains.insert(Self::extract_domain(tool));
        }

        // If there are more than 2 distinct domains among the tools, we consider that "clear domain separation"
        // and split into multi-agent to avoid overwhelming a single agent with too many disparate contexts.
        if domains.len() > 2 {
            return SplitDecision::MultiAgent;
        }

        SplitDecision::SingleAgent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::tools::{Tool, ToolExecutor};
    use crate::types::ToolError;

    struct DummyExecutor;
    #[async_trait::async_trait]
    impl ToolExecutor for DummyExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            Ok("dummy".to_string())
        }
    }

    fn make_tool(name: &str, desc: &str) -> Tool {
        Tool {
            name: name.to_string(),
            description: desc.to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(DummyExecutor),
        }
    }

    #[test]
    fn test_single_agent_under_10_tools_one_domain() {
        let tools = vec![
            make_tool("bash_1", "execute bash"),
            make_tool("bash_2", "execute bash"),
            make_tool("read_1", "read file"),
            make_tool("read_2", "read file"),
            make_tool("grep_1", "grep file"),
        ]; // 5 tools, domain: filesystem

        assert_eq!(AgentSplitter::decide_split(&tools), SplitDecision::SingleAgent);
    }

    #[test]
    fn test_multi_agent_over_10_tools() {
        let mut tools = vec![];
        for i in 0..11 {
            tools.push(make_tool(&format!("bash_{}", i), "bash command"));
        } // 11 tools, domain: filesystem

        assert_eq!(AgentSplitter::decide_split(&tools), SplitDecision::MultiAgent);
    }

    #[test]
    fn test_single_agent_2_domains() {
        let tools = vec![
            make_tool("bash_1", "execute bash"), // filesystem
            make_tool("bash_2", "execute bash"), // filesystem
            make_tool("git_add", "git repo"),    // version_control
            make_tool("git_commit", "git repo"), // version_control
        ];

        assert_eq!(AgentSplitter::decide_split(&tools), SplitDecision::SingleAgent);
    }

    #[test]
    fn test_multi_agent_3_domains() {
        let tools = vec![
            make_tool("bash_1", "execute bash"), // filesystem
            make_tool("git_add", "git repo"),    // version_control
            make_tool("http_fetch", "web page"), // network
        ];

        assert_eq!(AgentSplitter::decide_split(&tools), SplitDecision::MultiAgent);
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(AgentSplitter::extract_domain(&make_tool("git_diff", "diff the repo")), "version_control");
        assert_eq!(AgentSplitter::extract_domain(&make_tool("read_file", "read a file")), "filesystem");
        assert_eq!(AgentSplitter::extract_domain(&make_tool("fetch_url", "http get")), "network");
        assert_eq!(AgentSplitter::extract_domain(&make_tool("query_db", "run sql statement")), "database");
        assert_eq!(AgentSplitter::extract_domain(&make_tool("something_else", "unknown")), "general");
    }
}
