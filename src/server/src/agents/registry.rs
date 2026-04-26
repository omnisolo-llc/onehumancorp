use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use crate::agents::provider::{Provider, ProviderType, Credentials, ClaudeProvider};

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
        // Add more providers here as they are implemented
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
#[allow(dead_code)]
pub struct ProviderInfo {
    pub r#type: ProviderType,
    pub description: String,
    pub recommended_roles: Vec<String>,
    pub is_authenticated: bool,
}
