use crate::agent::AgentRunConfig;
use crate::tools::Tool;
use std::sync::Arc;

/// Ruflo Unique Harness Innovations: 32+ Claude Code plugins
/// A basic plugin system implementation for Claude Code.

pub trait ClaudePlugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn get_tools(&self) -> Vec<Tool>;
    fn setup(&self, config: &mut AgentRunConfig) -> Result<(), String>;
}

pub struct PluginManager {
    plugins: Vec<Arc<dyn ClaudePlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register_plugin(&mut self, plugin: Arc<dyn ClaudePlugin>) {
        self.plugins.push(plugin);
    }

    pub fn setup_all(&self, config: &mut AgentRunConfig) -> Result<Vec<Tool>, String> {
        let mut all_tools = Vec::new();
        for plugin in &self.plugins {
            plugin.setup(config)?;
            let tools = plugin.get_tools();
            all_tools.extend(tools);
        }
        Ok(all_tools)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyPlugin;

    impl ClaudePlugin for DummyPlugin {
        fn name(&self) -> &str { "DummyPlugin" }
        fn description(&self) -> &str { "A dummy plugin" }
        fn get_tools(&self) -> Vec<Tool> {
            vec![] // Return empty tools for testing
        }
        fn setup(&self, config: &mut AgentRunConfig) -> Result<(), String> {
            config.developer_instructions.push_str("\n[DummyPlugin Setup]");
            Ok(())
        }
    }

    #[test]
    fn test_plugin_manager() {
        let mut manager = PluginManager::new();
        manager.register_plugin(Arc::new(DummyPlugin));

        let mut config = AgentRunConfig::default();
        let tools = manager.setup_all(&mut config).unwrap();

        assert!(config.developer_instructions.contains("[DummyPlugin Setup]"));
        assert_eq!(tools.len(), 0);
    }
}
