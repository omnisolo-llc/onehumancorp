use super::client::AsanaClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct AsanaProvider {
    client: Arc<AsanaClient>,
    metadata: ProviderMetadata,
}

impl AsanaProvider {
    pub fn new(access_token: String) -> Self {
        let client = AsanaClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "asana".to_string(),
                name: "Asana".to_string(),
                category: "project_management".to_string(),
                base_url: "https://app.asana.com/api/1.0".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<AsanaClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "asana".to_string(),
                name: "Asana".to_string(),
                category: "project_management".to_string(),
                base_url: "https://app.asana.com/api/1.0".to_string(),
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

    pub async fn get_workspaces(&self) -> Result<Vec<super::client::AsanaWorkspace>, String> {
        self.client.get_workspaces().await
    }

    pub async fn get_projects(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<super::client::AsanaProject>, String> {
        self.client.get_projects(workspace_id).await
    }

    pub async fn create_project(
        &self,
        workspace_id: &str,
        name: &str,
        notes: Option<&str>,
    ) -> Result<super::client::AsanaProject, String> {
        self.client.create_project(workspace_id, name, notes).await
    }

    pub async fn get_tasks(
        &self,
        project_id: &str,
        completed_since: Option<&str>,
    ) -> Result<Vec<super::client::AsanaTask>, String> {
        self.client.get_tasks(project_id, completed_since).await
    }

    pub async fn create_task(
        &self,
        project_id: &str,
        name: &str,
        notes: Option<&str>,
        assignee: Option<&str>,
        due_date: Option<&str>,
    ) -> Result<super::client::AsanaTask, String> {
        self.client
            .create_task(project_id, name, notes, assignee, due_date)
            .await
    }

    pub async fn update_task(
        &self,
        task_id: &str,
        fields: &serde_json::Value,
    ) -> Result<super::client::AsanaTask, String> {
        self.client.update_task(task_id, fields).await
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), String> {
        self.client.complete_task(task_id).await
    }

    pub async fn get_task_comments(
        &self,
        task_id: &str,
    ) -> Result<Vec<super::client::AsanaComment>, String> {
        self.client.get_task_comments(task_id).await
    }

    pub async fn add_comment(
        &self,
        task_id: &str,
        text: &str,
    ) -> Result<super::client::AsanaComment, String> {
        self.client.add_comment(task_id, text).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asana_provider_new() {
        let provider = AsanaProvider::new("test-token".to_string());
        assert_eq!(provider.metadata.id, "asana");
        assert_eq!(provider.metadata.category, "project_management");
    }

    #[test]
    fn test_asana_provider_to_integration_provider() {
        let provider = AsanaProvider::new("test-token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "asana");
    }

    #[test]
    fn test_asana_provider_with_client() {
        let client = Arc::new(AsanaClient::new("test-token".to_string()));
        let provider = AsanaProvider::with_client(client);
        assert_eq!(provider.metadata.id, "asana");
    }
}
