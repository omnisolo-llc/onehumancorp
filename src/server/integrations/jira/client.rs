use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraIssue {
    pub key: String,
    pub id: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub assignee: String,
    #[serde(default)]
    pub issue_type: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraProject {
    pub key: String,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub project_type_key: String,
}

#[derive(Debug, Deserialize)]
struct JiraCreateIssueResponse {
    id: String,
    key: String,
}

#[derive(Debug, Deserialize)]
struct JiraIssueResponse {
    id: String,
    key: String,
    fields: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct JiraSearchResponse {
    issues: Vec<JiraIssueResponse>,
    total: u32,
}

#[derive(Debug, Deserialize)]
struct JiraTransitionsResponse {
    transitions: Vec<JiraTransition>,
}

#[derive(Debug, Deserialize)]
struct JiraTransition {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct JiraErrorResponse {
    #[serde(default)]
    error_messages: Vec<String>,
    #[serde(default)]
    errors: std::collections::HashMap<String, String>,
}

pub struct JiraClient {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
    http_client: Client,
}

impl JiraClient {
    pub fn new(base_url: String, email: String, api_token: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            email,
            api_token,
            http_client: Client::new(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(base_url: String, email: String, api_token: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            email,
            api_token,
            http_client: Client::new(),
        }
    }

    fn basic_auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.email, self.api_token);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials.as_bytes());
        format!("Basic {}", encoded)
    }

    fn api_base(&self) -> String {
        format!("{}/rest/api/2", self.base_url)
    }

    async fn check_error_response(&self, resp: reqwest::Response) -> Result<reqwest::Response, String> {
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable>".to_string());

            if let Ok(err) = serde_json::from_str::<JiraErrorResponse>(&text) {
                let msgs = err.error_messages.join(", ");
                let errors: Vec<String> = err.errors.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                let all_errors = [msgs, errors.join(", ")].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join("; ");
                return Err(format!("Jira API error {} [{}]: {}", status, text, all_errors));
            }

            return Err(format!("Jira API error {}: {}", status, text));
        }
        Ok(resp)
    }

    fn parse_issue_from_response(resp: &JiraIssueResponse) -> JiraIssue {
        let fields = &resp.fields;
        JiraIssue {
            key: resp.key.clone(),
            id: resp.id.clone(),
            summary: fields["summary"].as_str().unwrap_or("").to_string(),
            description: fields["description"].as_str().unwrap_or("").to_string(),
            status: fields["status"]["name"].as_str().unwrap_or("").to_string(),
            priority: fields["priority"]["name"].as_str().unwrap_or("").to_string(),
            assignee: fields["assignee"]["displayName"]
                .as_str()
                .or_else(|| fields["assignee"]["name"].as_str())
                .unwrap_or("")
                .to_string(),
            issue_type: fields["issuetype"]["name"].as_str().unwrap_or("").to_string(),
            created: fields["created"].as_str().unwrap_or("").to_string(),
            updated: fields["updated"].as_str().unwrap_or("").to_string(),
        }
    }

    pub async fn create_issue(
        &self,
        project_key: &str,
        summary: &str,
        description: &str,
        issue_type: &str,
        priority: Option<&str>,
    ) -> Result<JiraIssue, String> {
        let mut fields = serde_json::json!({
            "project": {"key": project_key},
            "summary": summary,
            "description": description,
            "issuetype": {"name": issue_type},
        });

        if let Some(p) = priority {
            fields["priority"] = serde_json::json!({"name": p});
        }

        let payload = serde_json::json!({ "fields": fields });

        let url = format!("{}/issue", self.api_base());
        let resp = self
            .http_client
            .post(&url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating issue: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: JiraCreateIssueResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse create issue response: {}", e))?;

        Ok(JiraIssue {
            key: body.key,
            id: body.id,
            summary: summary.to_string(),
            description: description.to_string(),
            status: String::new(),
            priority: priority.unwrap_or("").to_string(),
            assignee: String::new(),
            issue_type: issue_type.to_string(),
            created: String::new(),
            updated: String::new(),
        })
    }

    pub async fn get_issue(&self, issue_key: &str) -> Result<JiraIssue, String> {
        let url = format!("{}/issue/{}?expand=renderedFields", self.api_base(), issue_key);
        let resp = self
            .http_client
            .get(&url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error getting issue: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: JiraIssueResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse get issue response: {}", e))?;

        Ok(Self::parse_issue_from_response(&body))
    }

    pub async fn update_issue(
        &self,
        issue_key: &str,
        fields: &serde_json::Value,
    ) -> Result<(), String> {
        let url = format!("{}/issue/{}", self.api_base(), issue_key);
        let resp = self
            .http_client
            .put(&url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "fields": fields }))
            .send()
            .await
            .map_err(|e| format!("Network error updating issue: {}", e))?;

        self.check_error_response(resp).await?;
        Ok(())
    }

    pub async fn transition_issue(
        &self,
        issue_key: &str,
        transition_name: &str,
    ) -> Result<(), String> {
        let transitions_url = format!("{}/issue/{}/transitions", self.api_base(), issue_key);
        let resp = self
            .http_client
            .get(&transitions_url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error getting transitions: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: JiraTransitionsResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse transitions response: {}", e))?;

        let transition = body
            .transitions
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(transition_name))
            .ok_or_else(|| {
                let available: Vec<&str> = body.transitions.iter().map(|t| t.name.as_str()).collect();
                format!(
                    "Transition '{}' not found. Available: {:?}",
                    transition_name, available
                )
            })?;

        let payload = serde_json::json!({ "transition": { "id": transition.id } });
        let resp = self
            .http_client
            .post(&transitions_url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error transitioning issue: {}", e))?;

        self.check_error_response(resp).await?;
        Ok(())
    }

    pub async fn search_issues(
        &self,
        jql: &str,
        max_results: u32,
        start_at: u32,
    ) -> Result<(Vec<JiraIssue>, u32), String> {
        let payload = serde_json::json!({
            "jql": jql,
            "maxResults": max_results,
            "startAt": start_at,
        });

        let url = format!("{}/search", self.api_base());
        let resp = self
            .http_client
            .post(&url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error searching issues: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: JiraSearchResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse search response: {}", e))?;

        let issues = body.issues.iter().map(Self::parse_issue_from_response).collect();
        Ok((issues, body.total))
    }

    pub async fn add_comment(&self, issue_key: &str, body: &str) -> Result<(), String> {
        let url = format!("{}/issue/{}/comment", self.api_base(), issue_key);
        let resp = self
            .http_client
            .post(&url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "body": body }))
            .send()
            .await
            .map_err(|e| format!("Network error adding comment: {}", e))?;

        self.check_error_response(resp).await?;
        Ok(())
    }

    pub async fn get_projects(&self) -> Result<Vec<JiraProject>, String> {
        let url = format!("{}/project", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error getting projects: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: Vec<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse projects response: {}", e))?;

        let projects = body
            .iter()
            .map(|p| JiraProject {
                key: p["key"].as_str().unwrap_or("").to_string(),
                id: p["id"].as_str().unwrap_or("").to_string(),
                name: p["name"].as_str().unwrap_or("").to_string(),
                project_type_key: p["projectTypeKey"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        Ok(projects)
    }

    pub async fn get_issue_types(&self, project_key: &str) -> Result<Vec<String>, String> {
        let url = format!("{}/project/{}/statuses", self.api_base(), project_key);
        let resp = self
            .http_client
            .get(&url)
            .header("Authorization", self.basic_auth_header())
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error getting issue types: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: Vec<serde_json::Value> = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse issue types response: {}", e))?;

        let types = body
            .iter()
            .filter_map(|t| t["name"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_server(
        response_body: &'static str,
    ) -> (String, oneshot::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let (request_tx, request_rx) = oneshot::channel();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let mut header_end = None;
            let mut content_length = 0_usize;

            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(read > 0, "client closed connection before sending request");
                request.extend_from_slice(&buffer[..read]);

                if header_end.is_none() {
                    if let Some(index) =
                        request.windows(4).position(|window| window == b"\r\n\r\n")
                    {
                        header_end = Some(index + 4);
                        let headers = String::from_utf8_lossy(&request[..index]);
                        content_length = headers
                            .lines()
                            .find_map(|line| {
                                line.strip_prefix("content-length: ")
                                    .or_else(|| line.strip_prefix("Content-Length: "))
                            })
                            .and_then(|value| value.trim().parse::<usize>().ok())
                            .unwrap_or(0);
                    }
                }

                if let Some(body_start) = header_end {
                    if request.len() >= body_start + content_length {
                        break;
                    }
                }
            }

            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
            request_tx.send(String::from_utf8(request).unwrap()).unwrap();
        });

        (base_url, request_rx)
    }

    fn request_body(request: &str) -> serde_json::Value {
        let (_, body) = request.split_once("\r\n\r\n").unwrap();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn create_issue_returns_new_issue() {
        let response = r#"{"id": "10001", "key": "PROJ-123"}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        let issue = client
            .create_issue("PROJ", "Test summary", "Test description", "Bug", Some("High"))
            .await
            .unwrap();
        assert_eq!(issue.key, "PROJ-123");
        assert_eq!(issue.id, "10001");
        assert_eq!(issue.summary, "Test summary");
        assert_eq!(issue.issue_type, "Bug");
        assert_eq!(issue.priority, "High");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /rest/api/2/issue"));
        assert!(request.contains("authorization: Basic "));
        let body = request_body(&request);
        assert_eq!(body["fields"]["project"]["key"], "PROJ");
        assert_eq!(body["fields"]["summary"], "Test summary");
        assert_eq!(body["fields"]["issuetype"]["name"], "Bug");
    }

    #[tokio::test]
    async fn get_issue_returns_parsed_issue() {
        let response = r#"{
            "id": "10002",
            "key": "PROJ-456",
            "fields": {
                "summary": "Login broken",
                "description": "Users cannot log in",
                "status": {"name": "In Progress"},
                "priority": {"name": "Critical"},
                "assignee": {"displayName": "Jane Doe"},
                "issuetype": {"name": "Bug"},
                "created": "2025-01-15T10:30:00.000+0000",
                "updated": "2025-01-16T12:00:00.000+0000"
            }
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        let issue = client.get_issue("PROJ-456").await.unwrap();
        assert_eq!(issue.key, "PROJ-456");
        assert_eq!(issue.summary, "Login broken");
        assert_eq!(issue.status, "In Progress");
        assert_eq!(issue.priority, "Critical");
        assert_eq!(issue.assignee, "Jane Doe");
        assert_eq!(issue.issue_type, "Bug");

        let request = request_rx.await.unwrap();
        assert!(request.contains("GET /rest/api/2/issue/PROJ-456?expand=renderedFields"));
    }

    #[tokio::test]
    async fn update_issue_sends_fields() {
        let response = r#"{}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        let fields = serde_json::json!({
            "summary": "Updated summary",
            "priority": {"name": "Low"}
        });

        client.update_issue("PROJ-789", &fields).await.unwrap();

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("PUT /rest/api/2/issue/PROJ-789"));
        let body = request_body(&request);
        assert_eq!(body["fields"]["summary"], "Updated summary");
        assert_eq!(body["fields"]["priority"]["name"], "Low");
    }

    #[tokio::test]
    async fn transition_issue_finds_matching_transition() {
        let transitions_response = r#"{
            "transitions": [
                {"id": "21", "name": "In Progress"},
                {"id": "31", "name": "Done"},
                {"id": "41", "name": "To Do"}
            ]
        }"#;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let (transitions_tx, transitions_rx) = oneshot::channel::<String>();
        let (transition_tx, transition_rx) = oneshot::channel::<String>();

        tokio::spawn(async move {
            // First connection: GET transitions
            {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                let mut buffer = [0_u8; 4096];
                let mut header_end = None;
                let mut content_length = 0_usize;

                loop {
                    let read = stream.read(&mut buffer).await.unwrap();
                    request.extend_from_slice(&buffer[..read]);

                    if header_end.is_none() {
                        if let Some(index) =
                            request.windows(4).position(|window| window == b"\r\n\r\n")
                        {
                            header_end = Some(index + 4);
                            let headers = String::from_utf8_lossy(&request[..index]);
                            content_length = headers
                                .lines()
                                .find_map(|line| {
                                    line.strip_prefix("content-length: ")
                                        .or_else(|| line.strip_prefix("Content-Length: "))
                                })
                                .and_then(|value| value.trim().parse::<usize>().ok())
                                .unwrap_or(0);
                        }
                    }

                    if let Some(body_start) = header_end {
                        if request.len() >= body_start + content_length {
                            break;
                        }
                    }
                }

                let resp_body = transitions_response;
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    resp_body.len(),
                    resp_body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
                transitions_tx.send(String::from_utf8(request).unwrap()).unwrap();
            }

            // Second connection: POST transition
            {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                let mut buffer = [0_u8; 4096];
                let mut header_end = None;
                let mut content_length = 0_usize;

                loop {
                    let read = stream.read(&mut buffer).await.unwrap();
                    request.extend_from_slice(&buffer[..read]);

                    if header_end.is_none() {
                        if let Some(index) =
                            request.windows(4).position(|window| window == b"\r\n\r\n")
                        {
                            header_end = Some(index + 4);
                            let headers = String::from_utf8_lossy(&request[..index]);
                            content_length = headers
                                .lines()
                                .find_map(|line| {
                                    line.strip_prefix("content-length: ")
                                        .or_else(|| line.strip_prefix("Content-Length: "))
                                })
                                .and_then(|value| value.trim().parse::<usize>().ok())
                                .unwrap_or(0);
                        }
                    }

                    if let Some(body_start) = header_end {
                        if request.len() >= body_start + content_length {
                            break;
                        }
                    }
                }

                let resp_body = r#"{}"#;
                let response = format!(
                    "HTTP/1.1 204 No Content\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    resp_body.len(),
                    resp_body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
                transition_tx.send(String::from_utf8(request).unwrap()).unwrap();
            }
        });

        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        client
            .transition_issue("PROJ-100", "In Progress")
            .await
            .unwrap();

        let _transitions_request = transitions_rx.await.unwrap();
        let transition_request = transition_rx.await.unwrap();
        assert!(transition_request.contains("POST /rest/api/2/issue/PROJ-100/transitions"));
        let body = request_body(&transition_request);
        assert_eq!(body["transition"]["id"], "21");
    }

    #[tokio::test]
    async fn transition_issue_returns_error_for_unknown_transition() {
        let transitions_response = r#"{
            "transitions": [
                {"id": "21", "name": "In Progress"}
            ]
        }"#;
        let (base_url, _) = start_server(transitions_response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        let err = client
            .transition_issue("PROJ-100", "Nonexistent")
            .await
            .unwrap_err();
        assert!(err.contains("Transition 'Nonexistent' not found"));
        assert!(err.contains("In Progress"));
    }

    #[tokio::test]
    async fn search_issues_returns_paginated_results() {
        let response = r#"{
            "issues": [
                {
                    "id": "10010",
                    "key": "PROJ-10",
                    "fields": {
                        "summary": "Bug one",
                        "status": {"name": "Open"},
                        "priority": {"name": "Medium"},
                        "issuetype": {"name": "Bug"}
                    }
                },
                {
                    "id": "10011",
                    "key": "PROJ-11",
                    "fields": {
                        "summary": "Bug two",
                        "status": {"name": "Closed"},
                        "priority": {"name": "Low"},
                        "issuetype": {"name": "Task"}
                    }
                }
            ],
            "total": 50
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        let (issues, total) = client
            .search_issues("project = PROJ AND status != Closed", 25, 0)
            .await
            .unwrap();
        assert_eq!(issues.len(), 2);
        assert_eq!(total, 50);
        assert_eq!(issues[0].key, "PROJ-10");
        assert_eq!(issues[0].summary, "Bug one");
        assert_eq!(issues[1].key, "PROJ-11");
        assert_eq!(issues[1].issue_type, "Task");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /rest/api/2/search"));
        let body = request_body(&request);
        assert_eq!(body["jql"], "project = PROJ AND status != Closed");
        assert_eq!(body["maxResults"], 25);
        assert_eq!(body["startAt"], 0);
    }

    #[tokio::test]
    async fn add_comment_posts_body() {
        let response = r#"{}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        client
            .add_comment("PROJ-55", "This is a comment")
            .await
            .unwrap();

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /rest/api/2/issue/PROJ-55/comment"));
        let body = request_body(&request);
        assert_eq!(body["body"], "This is a comment");
    }

    #[tokio::test]
    async fn get_projects_returns_projects() {
        let response = r#"[
            {"key": "PROJ", "id": "10000", "name": "My Project", "projectTypeKey": "software"},
            {"key": "OPS", "id": "10001", "name": "Operations", "projectTypeKey": "business"}
        ]"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        let projects = client.get_projects().await.unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].key, "PROJ");
        assert_eq!(projects[0].name, "My Project");
        assert_eq!(projects[0].project_type_key, "software");
        assert_eq!(projects[1].key, "OPS");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /rest/api/2/project"));
    }

    #[tokio::test]
    async fn get_issue_types_returns_types() {
        let response = r#"[
            {"name": "Bug", "id": "10001"},
            {"name": "Story", "id": "10002"},
            {"name": "Task", "id": "10003"}
        ]"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );

        let types = client.get_issue_types("PROJ").await.unwrap();
        assert_eq!(types.len(), 3);
        assert_eq!(types[0], "Bug");
        assert_eq!(types[1], "Story");
        assert_eq!(types[2], "Task");

        let request = request_rx.await.unwrap();
        assert!(request.contains("GET /rest/api/2/project/PROJ/statuses"));
    }

    #[tokio::test]
    async fn handles_jira_error_response() {
        let error_body = r#"{"errorMessages": ["Issue does not exist"], "errors": {}}"#;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let response = format!(
                "HTTP/1.1 404 Not Found\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                error_body.len(),
                error_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );
        let error = client.get_issue("NONEXISTENT-1").await.unwrap_err();
        assert!(error.contains("404"));
        assert!(error.contains("Issue does not exist"));
    }

    #[tokio::test]
    async fn handles_field_level_errors() {
        let error_body = r#"{"errorMessages": [], "errors": {"summary": "Field is required"}}"#;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let response = format!(
                "HTTP/1.1 400 Bad Request\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                error_body.len(),
                error_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let client = JiraClient::with_base_url_for_test(
            base_url,
            "test@example.com".to_string(),
            "test-token".to_string(),
        );
        let error = client
            .create_issue("PROJ", "", "", "Bug", None)
            .await
            .unwrap_err();
        assert!(error.contains("summary: Field is required"));
    }
}
