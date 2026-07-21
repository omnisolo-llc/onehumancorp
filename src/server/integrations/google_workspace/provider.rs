use super::client::GoogleWorkspaceClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleWorkspaceProvider {
    client: Arc<GoogleWorkspaceClient>,
    metadata: ProviderMetadata,
}

impl GoogleWorkspaceProvider {
    pub fn new(access_token: String) -> Self {
        let client = GoogleWorkspaceClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "google_workspace".to_string(),
                name: "Google Workspace".to_string(),
                category: "productivity".to_string(),
                base_url: "https://www.googleapis.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<GoogleWorkspaceClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "google_workspace".to_string(),
                name: "Google Workspace".to_string(),
                category: "productivity".to_string(),
                base_url: "https://www.googleapis.com".to_string(),
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

    // ── Drive ─────────────────────────────────────────────────────

    pub async fn drive_list_files(
        &self,
        folder_id: &str,
        page_size: u32,
    ) -> Result<Vec<super::client::DriveFile>, String> {
        self.client.list_files(folder_id, page_size).await
    }

    pub async fn drive_get_file(
        &self,
        file_id: &str,
    ) -> Result<super::client::DriveFile, String> {
        self.client.get_file(file_id).await
    }

    pub async fn drive_create_file(
        &self,
        name: &str,
        mime_type: &str,
        parent_id: &str,
        content: &[u8],
    ) -> Result<super::client::DriveFile, String> {
        self.client
            .create_file(name, mime_type, parent_id, content)
            .await
    }

    // ── Sheets ────────────────────────────────────────────────────

    pub async fn sheets_read_range(
        &self,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<Vec<Vec<String>>, String> {
        self.client.read_range(spreadsheet_id, range).await
    }

    pub async fn sheets_write_range(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: &[Vec<String>],
    ) -> Result<(), String> {
        self.client.write_range(spreadsheet_id, range, values).await
    }

    pub async fn sheets_create_spreadsheet(
        &self,
        title: &str,
    ) -> Result<String, String> {
        self.client.create_spreadsheet(title).await
    }

    // ── Gmail ─────────────────────────────────────────────────────

    pub async fn gmail_send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<String, String> {
        self.client.send_email(to, subject, body).await
    }

    pub async fn gmail_list_messages(
        &self,
        query: &str,
        max_results: u32,
    ) -> Result<Vec<super::client::GmailMessage>, String> {
        self.client.list_messages(query, max_results).await
    }

    pub async fn gmail_get_message(
        &self,
        message_id: &str,
    ) -> Result<super::client::GmailMessage, String> {
        self.client.get_message(message_id).await
    }
}
