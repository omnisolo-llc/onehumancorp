#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use crate::provider::{
    Provider, ProviderType, Credentials, ClaudeProvider, GeminiProvider,
    OpenCodeProvider, OpenClawProvider, IronClawProvider, MiniMaxiProvider,
    BuiltinProvider, ScoutProvider
};

pub struct Registry {
    providers: RwLock<HashMap<ProviderType, Arc<dyn Provider>>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            providers: RwLock::new(HashMap::new()),
        }
    }

    pub fn default_registry() -> Self {
        let r = Self::new();
        r.register(Arc::new(ClaudeProvider::new()));
        r.register(Arc::new(GeminiProvider::new()));
        r.register(Arc::new(OpenCodeProvider::new()));
        r.register(Arc::new(OpenClawProvider::new()));
        r.register(Arc::new(IronClawProvider::new()));
        r.register(Arc::new(MiniMaxiProvider::new()));
        r.register(Arc::new(BuiltinProvider::new()));
        r.register(Arc::new(ScoutProvider::new()));
        r
    }

    pub fn register(&self, p: Arc<dyn Provider>) {
        let mut providers = self.providers.write().unwrap();
        providers.insert(p.provider_type(), p);
    }

    pub fn get(&self, t: ProviderType) -> Option<Arc<dyn Provider>> {
        let providers = self.providers.read().unwrap();
        providers.get(&t).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn Provider>> {
        let providers = self.providers.read().unwrap();
        let mut out: Vec<Arc<dyn Provider>> = Vec::new();
        
        // Stable ordering similar to Go implementation
        let ordered = vec![
            ProviderType::Claude,
            ProviderType::Gemini,
            ProviderType::OpenCode,
            ProviderType::OpenClaw,
            ProviderType::IronClaw,
            ProviderType::Builtin,
            ProviderType::Scout,
            ProviderType::MiniMaxi,
        ];

        let mut seen = std::collections::HashSet::new();
        for t in ordered {
            if let Some(p) = providers.get(&t) {
                out.push(p.clone());
                seen.insert(t);
            }
        }

        for (t, p) in providers.iter() {
            if !seen.contains(t) {
                out.push(p.clone());
            }
        }

        out
    }

    pub fn authenticate(&self, t: ProviderType, creds: Credentials) -> Result<(), String> {
        let p = self.get(t.clone()).ok_or_else(|| format!("unknown provider type: {:?}", t))?;
        p.authenticate(creds)
    }

    pub fn infos(&self) -> Vec<ProviderInfo> {
        self.all().iter().map(|p| ProviderInfo {
            r#type: p.provider_type(),
            description: p.description(),
            recommended_roles: p.supported_roles(),
            is_authenticated: p.is_authenticated(),
        }).collect()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderInfo {
    pub r#type: ProviderType,
    pub description: String,
    pub recommended_roles: Vec<String>,
    pub is_authenticated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::Credentials;

    #[test]
    fn test_provider_get_credentials() {
        let registry = Registry::default_registry();
        let creds = Credentials {
            api_key: "test-key".to_string(),
            oauth_token: "".to_string(),
            extra: std::collections::HashMap::new(),
        };

        let test_cases = vec![
            ProviderType::Claude,
            ProviderType::Gemini,
            ProviderType::OpenCode,
            ProviderType::OpenClaw,
            ProviderType::IronClaw,
            ProviderType::MiniMaxi,
        ];

        for t in test_cases {
            let provider = registry.get(t.clone()).expect("provider should exist");
            provider.authenticate(creds.clone()).expect("auth should succeed");
            assert_eq!(provider.get_credentials().api_key, "test-key");
            assert!(provider.is_authenticated());
        }
    }

    #[test]
    fn test_builtin_provider_always_authenticated() {
        let registry = Registry::default_registry();
        let provider = registry.get(ProviderType::Builtin).expect("provider should exist");
        assert!(provider.is_authenticated());
        
        let creds = Credentials {
            api_key: "some-key".to_string(),
            oauth_token: "".to_string(),
            extra: std::collections::HashMap::new(),
        };
        provider.authenticate(creds).expect("auth should succeed");
        assert!(provider.is_authenticated());
        assert!(provider.get_credentials().api_key.is_empty()); // Builtin doesn't store creds
    }

    #[test]
    fn test_hello_world_agent_example() {
        let registry = Registry::default_registry();
        let provider = registry.get(ProviderType::Builtin).expect("Built-in provider not found");
        tracing::info!("Successfully loaded provider: {}", provider.provider_type());
        tracing::info!("Description: {}", provider.description());
        tracing::info!("Is Authenticated: {}", provider.is_authenticated());
        tracing::info!("Hello World! The agent provider is ready to use with zero configuration.");
    }
}
