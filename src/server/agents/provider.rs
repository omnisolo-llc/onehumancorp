use async_trait::async_trait;
use std::sync::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderType {
    Claude,
    Gemini,
    OpenCode,
    OpenClaw,
    IronClaw,
    Builtin,
    Scout,
    MiniMaxi,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProviderType::Claude => "claude",
            ProviderType::Gemini => "gemini",
            ProviderType::OpenCode => "opencode",
            ProviderType::OpenClaw => "openclaw",
            ProviderType::IronClaw => "ironclaw",
            ProviderType::Builtin => "builtin",
            ProviderType::Scout => "scout",
            ProviderType::MiniMaxi => "minimaxi",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Credentials {
    pub api_key: String,
    pub oauth_token: String,
    pub extra: HashMap<String, String>,
}

impl Credentials {
    pub fn is_empty(&self) -> bool {
        self.api_key.is_empty() && self.oauth_token.is_empty()
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn provider_type(&self) -> ProviderType;
    fn description(&self) -> String;
    fn supported_roles(&self) -> Vec<String>;
    fn authenticate(&self, creds: Credentials) -> Result<(), String>;
    fn get_credentials(&self) -> Credentials;
    fn is_authenticated(&self) -> bool;
}

pub struct BaseProvider {
    cred: RwLock<Credentials>,
}

impl BaseProvider {
    pub fn new() -> Self {
        BaseProvider {
            cred: RwLock::new(Credentials::default()),
        }
    }

    pub fn store(&self, cred: Credentials) {
        let mut c = self.cred.write().unwrap();
        *c = cred;
    }

    pub fn load(&self) -> Credentials {
        let c = self.cred.read().unwrap();
        c.clone()
    }
}



