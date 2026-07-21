use super::client::GitHubClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GitHubProvider {
    client: Arc<GitHubClient>,
    metadata: ProviderMetadata,
}

impl GitHubProvider {
    pub fn new(access_token: String) -> Self {
        let client = GitHubClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "github_api".to_string(),
                name: "GitHub API".to_string(),
                category: "code_management".to_string(),
                base_url: "https://api.github.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<GitHubClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "github_api".to_string(),
                name: "GitHub API".to_string(),
                category: "code_management".to_string(),
                base_url: "https://api.github.com".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            },
        }
    }

    pub async fn get_repositories(
        &self,
        sort: &str,
        per_page: u32,
    ) -> Result<Vec<super::client::GitHubRepo>, String> {
        self.client.get_repositories(sort, per_page).await
    }

    pub async fn create_repository(
        &self,
        name: &str,
        description: &str,
        private: bool,
    ) -> Result<super::client::GitHubRepo, String> {
        self.client.create_repository(name, description, private).await
    }

    pub async fn get_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
    ) -> Result<Vec<super::client::GHPullRequest>, String> {
        self.client.get_pull_requests(owner, repo, state).await
    }

    pub async fn create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        head: &str,
        base: &str,
        body: &str,
    ) -> Result<super::client::GHPullRequest, String> {
        self.client
            .create_pull_request(owner, repo, title, head, base, body)
            .await
    }

    pub async fn get_issues(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        per_page: u32,
    ) -> Result<Vec<super::client::GHIssue>, String> {
        self.client.get_issues(owner, repo, state, per_page).await
    }

    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
        labels: &[String],
    ) -> Result<super::client::GHIssue, String> {
        self.client.create_issue(owner, repo, title, body, labels).await
    }

    pub async fn get_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<String, String> {
        self.client.get_file_contents(owner, repo, path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_provider_new() {
        let provider = GitHubProvider::new("ghp_test".to_string());
        assert_eq!(provider.metadata.id, "github_api");
        assert_eq!(provider.metadata.category, "code_management");
    }

    #[test]
    fn test_github_provider_to_integration_provider() {
        let provider = GitHubProvider::new("ghp_test".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "github_api");
    }

    #[test]
    fn test_github_provider_with_client() {
        let client = Arc::new(GitHubClient::new("ghp_test".to_string()));
        let provider = GitHubProvider::with_client(client);
        assert_eq!(provider.metadata.id, "github_api");
    }
}
