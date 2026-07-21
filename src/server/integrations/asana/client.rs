use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

const BASE_URL: &str = "https://app.asana.com/api/1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsanaWorkspace {
    pub gid: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsanaProject {
    pub gid: String,
    pub name: String,
    pub notes: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsanaTask {
    pub gid: String,
    pub name: String,
    pub notes: Option<String>,
    pub completed: Option<bool>,
    pub assignee: Option<AsanaUser>,
    pub due_on: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsanaUser {
    pub gid: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsanaComment {
    pub gid: String,
    pub text: Option<String>,
    pub created_at: Option<String>,
    pub author: Option<AsanaUser>,
}

#[derive(Debug, Deserialize)]
struct AsanaEnvelope<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct AsanaListEnvelope<T> {
    data: Vec<T>,
}

pub struct AsanaClient {
    access_token: String,
}

impl AsanaClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }

    pub async fn get_workspaces(&self) -> Result<Vec<AsanaWorkspace>, String> {
        let url = format!("{}/workspaces", BASE_URL);
        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaListEnvelope<AsanaWorkspace> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }

    pub async fn get_projects(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<AsanaProject>, String> {
        let url = format!("{}/workspaces/{}/projects", BASE_URL, workspace_id);
        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaListEnvelope<AsanaProject> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }

    pub async fn create_project(
        &self,
        workspace_id: &str,
        name: &str,
        notes: Option<&str>,
    ) -> Result<AsanaProject, String> {
        let url = format!("{}/projects", BASE_URL);
        let client = get_client();
        let mut payload = serde_json::json!({
            "data": {
                "workspace": workspace_id,
                "name": name,
            }
        });
        if let Some(n) = notes {
            payload["data"]["notes"] = serde_json::json!(n);
        }

        let res = client
            .post(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaEnvelope<AsanaProject> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }

    pub async fn get_tasks(
        &self,
        project_id: &str,
        completed_since: Option<&str>,
    ) -> Result<Vec<AsanaTask>, String> {
        let url = format!("{}/projects/{}/tasks", BASE_URL, project_id);
        let client = get_client();
        let mut req = client
            .get(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json");
        if let Some(since) = completed_since {
            req = req.query(&[("completed_since", since)]);
        }

        let res = req.send().await.map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaListEnvelope<AsanaTask> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }

    pub async fn create_task(
        &self,
        project_id: &str,
        name: &str,
        notes: Option<&str>,
        assignee: Option<&str>,
        due_date: Option<&str>,
    ) -> Result<AsanaTask, String> {
        let url = format!("{}/tasks", BASE_URL);
        let client = get_client();
        let mut payload = serde_json::json!({
            "data": {
                "projects": [project_id],
                "name": name,
            }
        });
        if let Some(n) = notes {
            payload["data"]["notes"] = serde_json::json!(n);
        }
        if let Some(a) = assignee {
            payload["data"]["assignee"] = serde_json::json!(a);
        }
        if let Some(d) = due_date {
            payload["data"]["due_on"] = serde_json::json!(d);
        }

        let res = client
            .post(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaEnvelope<AsanaTask> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }

    pub async fn update_task(
        &self,
        task_id: &str,
        fields: &serde_json::Value,
    ) -> Result<AsanaTask, String> {
        let url = format!("{}/tasks/{}", BASE_URL, task_id);
        let client = get_client();
        let payload = serde_json::json!({ "data": fields });

        let res = client
            .put(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaEnvelope<AsanaTask> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), String> {
        let url = format!("{}/tasks/{}", BASE_URL, task_id);
        let client = get_client();
        let payload = serde_json::json!({
            "data": { "completed": true }
        });

        let res = client
            .put(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }
        Ok(())
    }

    pub async fn get_task_comments(
        &self,
        task_id: &str,
    ) -> Result<Vec<AsanaComment>, String> {
        let url = format!("{}/tasks/{}/stories", BASE_URL, task_id);
        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaListEnvelope<AsanaComment> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }

    pub async fn add_comment(
        &self,
        task_id: &str,
        text: &str,
    ) -> Result<AsanaComment, String> {
        let url = format!("{}/tasks/{}/stories", BASE_URL, task_id);
        let client = get_client();
        let payload = serde_json::json!({
            "data": { "text": text }
        });

        let res = client
            .post(&url)
            .header("Authorization", self.auth_header())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Asana API HTTP {}: {}", status, body));
        }

        let envelope: AsanaEnvelope<AsanaComment> = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(envelope.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::sync::Arc;

    fn start_mock_server(responses: Vec<String>) -> (Arc<TcpListener>, u16) {
        let listener = Arc::new(TcpListener::bind("127.0.0.1:0").unwrap());
        let port = listener.local_addr().unwrap().port();
        let listener_clone = Arc::clone(&listener);

        std::thread::spawn(move || {
            for resp_body in responses {
                if let Ok(stream) = listener_clone.accept() {
                    let mut stream = stream.0;
                    let reader = BufReader::new(stream.try_clone().unwrap());
                    let mut headers = Vec::new();
                    for line in reader.lines() {
                        match line {
                            Ok(line) if line.is_empty() => break,
                            Ok(line) => headers.push(line),
                            Err(_) => break,
                        }
                    }

                    let content_length = headers
                        .iter()
                        .find(|h| h.to_lowercase().starts_with("content-length:"))
                        .and_then(|h| h.split(':').nth(1))
                        .and_then(|v| v.trim().parse::<usize>().ok())
                        .unwrap_or(0);

                    let mut body = vec![0u8; content_length];
                    std::io::Read::read_exact(&mut stream, &mut body).unwrap();

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        resp_body.len(),
                        resp_body
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        });

        (listener, port)
    }

    #[test]
    fn test_asana_workspace_deserialize() {
        let json = serde_json::json!({
            "gid": "12345",
            "name": "My Workspace"
        });
        let ws: AsanaWorkspace = serde_json::from_value(json).unwrap();
        assert_eq!(ws.gid, "12345");
        assert_eq!(ws.name, "My Workspace");
    }

    #[test]
    fn test_asana_project_deserialize() {
        let json = serde_json::json!({
            "gid": "67890",
            "name": "Sprint 1",
            "notes": "Q3 planning",
            "archived": false
        });
        let proj: AsanaProject = serde_json::from_value(json).unwrap();
        assert_eq!(proj.gid, "67890");
        assert_eq!(proj.name, "Sprint 1");
        assert_eq!(proj.notes.as_deref(), Some("Q3 planning"));
    }

    #[test]
    fn test_asana_task_deserialize() {
        let json = serde_json::json!({
            "gid": "11111",
            "name": "Fix login bug",
            "notes": "Critical issue",
            "completed": false,
            "assignee": { "gid": "22222", "name": "Alice" },
            "due_on": "2026-08-01"
        });
        let task: AsanaTask = serde_json::from_value(json).unwrap();
        assert_eq!(task.gid, "11111");
        assert_eq!(task.name, "Fix login bug");
        assert_eq!(task.completed, Some(false));
        assert!(task.assignee.is_some());
        assert_eq!(task.due_on.as_deref(), Some("2026-08-01"));
    }

    #[test]
    fn test_asana_comment_deserialize() {
        let json = serde_json::json!({
            "gid": "33333",
            "text": "Looks good!",
            "created_at": "2026-07-21T10:00:00Z",
            "author": { "gid": "22222", "name": "Alice" }
        });
        let comment: AsanaComment = serde_json::from_value(json).unwrap();
        assert_eq!(comment.gid, "33333");
        assert_eq!(comment.text.as_deref(), Some("Looks good!"));
    }

    #[test]
    fn test_asana_client_new() {
        let client = AsanaClient::new("test-token".to_string());
        assert_eq!(client.access_token, "test-token");
    }

    #[tokio::test]
    async fn test_get_workspaces_mock() {
        let resp = serde_json::json!({
            "data": [
                { "gid": "12345", "name": "Workspace 1" },
                { "gid": "67890", "name": "Workspace 2" }
            ]
        })
        .to_string();

        let (_listener, port) = start_mock_server(vec![resp]);

        let url = format!("http://127.0.0.1:{}/workspaces", port);
        let res = get_client()
            .get(&url)
            .header("Authorization", "Bearer test-token")
            .header("Accept", "application/json")
            .send()
            .await
            .unwrap();
        let envelope: AsanaListEnvelope<AsanaWorkspace> = res.json().await.unwrap();
        assert_eq!(envelope.data.len(), 2);
        assert_eq!(envelope.data[0].name, "Workspace 1");
    }

    #[tokio::test]
    async fn test_create_project_mock() {
        let resp = serde_json::json!({
            "data": {
                "gid": "99999",
                "name": "New Project",
                "notes": "Test notes",
                "archived": false
            }
        })
        .to_string();

        let (_listener, port) = start_mock_server(vec![resp]);

        let url = format!("http://127.0.0.1:{}/projects", port);
        let payload = serde_json::json!({
            "data": {
                "workspace": "12345",
                "name": "New Project",
                "notes": "Test notes"
            }
        });
        let res = get_client()
            .post(&url)
            .header("Authorization", "Bearer test-token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .unwrap();
        let envelope: AsanaEnvelope<AsanaProject> = res.json().await.unwrap();
        assert_eq!(envelope.data.gid, "99999");
        assert_eq!(envelope.data.name, "New Project");
    }

    #[tokio::test]
    async fn test_complete_task_mock() {
        let resp = serde_json::json!({
            "data": { "gid": "11111", "completed": true }
        })
        .to_string();

        let (_listener, port) = start_mock_server(vec![resp]);

        let url = format!("http://127.0.0.1:{}/tasks/11111", port);
        let payload = serde_json::json!({
            "data": { "completed": true }
        });
        let res = get_client()
            .put(&url)
            .header("Authorization", "Bearer test-token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .unwrap();
        assert!(res.status().is_success());
    }
}
