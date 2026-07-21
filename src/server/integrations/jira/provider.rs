use super::client::JiraClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct JiraProvider {
    client: Arc<JiraClient>,
    metadata: ProviderMetadata,
}

impl JiraProvider {
    pub fn new(base_url: String, email: String, api_token: String) -> Self {
        let client = JiraClient::new(base_url.clone(), email, api_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "jira".to_string(),
                name: "Jira".to_string(),
                category: "project-management".to_string(),
                base_url,
            },
        }
    }

    pub fn with_client(client: Arc<JiraClient>, base_url: String) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "jira".to_string(),
                name: "Jira".to_string(),
                category: "project-management".to_string(),
                base_url,
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

    pub async fn create_issue(
        &self,
        project_key: &str,
        summary: &str,
        description: &str,
        issue_type: &str,
        priority: Option<&str>,
    ) -> Result<super::client::JiraIssue, String> {
        self.client
            .create_issue(project_key, summary, description, issue_type, priority)
            .await
    }

    pub async fn get_issue(&self, issue_key: &str) -> Result<super::client::JiraIssue, String> {
        self.client.get_issue(issue_key).await
    }

    pub async fn update_issue(
        &self,
        issue_key: &str,
        fields: &serde_json::Value,
    ) -> Result<(), String> {
        self.client.update_issue(issue_key, fields).await
    }

    pub async fn transition_issue(
        &self,
        issue_key: &str,
        transition_name: &str,
    ) -> Result<(), String> {
        self.client.transition_issue(issue_key, transition_name).await
    }

    pub async fn search_issues(
        &self,
        jql: &str,
        max_results: u32,
        start_at: u32,
    ) -> Result<(Vec<super::client::JiraIssue>, u32), String> {
        self.client.search_issues(jql, max_results, start_at).await
    }

    pub async fn add_comment(&self, issue_key: &str, body: &str) -> Result<(), String> {
        self.client.add_comment(issue_key, body).await
    }

    pub async fn get_projects(&self) -> Result<Vec<super::client::JiraProject>, String> {
        self.client.get_projects().await
    }

    pub async fn get_issue_types(&self, project_key: &str) -> Result<Vec<String>, String> {
        self.client.get_issue_types(project_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jira_provider_new() {
        let provider = JiraProvider::new(
            "https://mycompany.atlassian.net".to_string(),
            "admin@my.com".to_string(),
            "test-token".to_string(),
        );
        assert_eq!(provider.metadata.id, "jira");
        assert_eq!(provider.metadata.category, "project-management");
    }

    #[test]
    fn test_jira_provider_to_integration_provider() {
        let provider = JiraProvider::new(
            "https://mycompany.atlassian.net".to_string(),
            "admin@my.com".to_string(),
            "test-token".to_string(),
        );
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "jira");
    }

    #[test]
    fn test_jira_provider_with_client() {
        let client = Arc::new(JiraClient::new(
            "https://mycompany.atlassian.net".to_string(),
            "admin@my.com".to_string(),
            "test-token".to_string(),
        ));
        let provider =
            JiraProvider::with_client(client, "https://mycompany.atlassian.net".to_string());
        assert_eq!(provider.metadata.id, "jira");
    }
}
