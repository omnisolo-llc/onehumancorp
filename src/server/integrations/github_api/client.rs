use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

const BASE_URL: &str = "https://api.github.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub stargazers_count: u64,
    #[serde(default)]
    pub forks_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHPullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub user: Option<GHUser>,
    #[serde(default)]
    pub head: Option<GHPRBranch>,
    #[serde(default)]
    pub base: Option<GHPRBranch>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHUser {
    pub login: String,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHPRBranch {
    #[serde(default)]
    pub ref_name: Option<String>,
    #[serde(rename = "ref", default)]
    pub ref_: Option<String>,
    #[serde(default)]
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHIssue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub user: Option<GHUser>,
    #[serde(default)]
    pub labels: Vec<GHLabel>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHLabel {
    pub name: String,
    #[serde(default)]
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHFileContent {
    pub content: String,
    pub encoding: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
struct GHRepoList {
    items: Vec<GitHubRepo>,
}

#[derive(Debug, Deserialize)]
struct GHPRList {
    items: Vec<GHPullRequest>,
}

#[derive(Debug, Deserialize)]
struct GHIssueList {
    items: Vec<GHIssue>,
}

#[derive(Debug, Deserialize)]
struct GHError {
    message: String,
}

pub struct GitHubClient {
    access_token: String,
}

impl GitHubClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }

    #[cfg(test)]
    fn with_base_url_for_test(access_token: String, _base_url: String) -> Self {
        Self { access_token }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("GitHub access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", BASE_URL, path)
    }

    pub async fn get_repositories(
        &self,
        sort: &str,
        per_page: u32,
    ) -> Result<Vec<GitHubRepo>, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/user/repos?sort={}&per_page={}",
            BASE_URL, sort, per_page
        );

        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<Vec<GitHubRepo>>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn create_repository(
        &self,
        name: &str,
        description: &str,
        private: bool,
    ) -> Result<GitHubRepo, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/user/repos", BASE_URL);

        let payload = serde_json::json!({
            "name": name,
            "description": description,
            "private": private,
        });

        let client = get_client();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<GitHubRepo>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn get_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
    ) -> Result<Vec<GHPullRequest>, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/repos/{}/{}/pulls?state={}",
            BASE_URL, owner, repo, state
        );

        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<Vec<GHPullRequest>>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        head: &str,
        base: &str,
        body: &str,
    ) -> Result<GHPullRequest, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/repos/{}/{}/pulls", BASE_URL, owner, repo);

        let payload = serde_json::json!({
            "title": title,
            "head": head,
            "base": base,
            "body": body,
        });

        let client = get_client();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<GHPullRequest>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn get_issues(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        per_page: u32,
    ) -> Result<Vec<GHIssue>, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/repos/{}/{}/issues?state={}&per_page={}",
            BASE_URL, owner, repo, state, per_page
        );

        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<Vec<GHIssue>>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
        labels: &[String],
    ) -> Result<GHIssue, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/repos/{}/{}/issues", BASE_URL, owner, repo);

        let payload = serde_json::json!({
            "title": title,
            "body": body,
            "labels": labels,
        });

        let client = get_client();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<GHIssue>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn get_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<String, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/repos/{}/{}/contents/{}", BASE_URL, owner, repo, path);

        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            let file: GHFileContent = res
                .json()
                .await
                .map_err(|e| format!("response parse error: {}", e))?;

            if file.encoding == "base64" {
                use base64::Engine;
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(&file.content)
                    .map_err(|e| format!("base64 decode error: {}", e))?;
                String::from_utf8(decoded).map_err(|e| format!("utf8 decode error: {}", e))
            } else {
                Ok(file.content)
            }
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_github_server(response_body: &'static str) -> (String, oneshot::Receiver<String>) {
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
    async fn get_repositories_sends_correct_request() {
        let response = r#"[
            {"id": 1, "name": "my-repo", "full_name": "user/my-repo", "private": false, "html_url": "https://github.com/user/my-repo", "created_at": "2026-01-01", "updated_at": "2026-01-02"}
        ]"#;
        let (base_url, request_rx) = start_github_server(response).await;

        let client = GitHubClient::with_base_url_for_test("ghp_test".to_string(), base_url);
        let repos = client.get_repositories("updated", 10).await.unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "my-repo");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /user/repos?sort=updated&per_page=10 HTTP/1.1"));
        assert!(request.contains("authorization: Bearer ghp_test"));
        assert!(request.contains("accept: application/vnd.github.v3+json"));
    }

    #[tokio::test]
    async fn create_repository_sends_post() {
        let response = r#"{"id": 2, "name": "new-repo", "full_name": "user/new-repo", "private": true, "html_url": "https://github.com/user/new-repo", "created_at": "2026-01-01", "updated_at": "2026-01-01"}"#;
        let (base_url, request_rx) = start_github_server(response).await;

        let client = GitHubClient::with_base_url_for_test("ghp_test".to_string(), base_url);
        let repo = client
            .create_repository("new-repo", "A test repo", true)
            .await
            .unwrap();

        assert_eq!(repo.name, "new-repo");
        assert!(repo.private);

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /user/repos HTTP/1.1"));

        let body = request_body(&request);
        assert_eq!(body["name"], "new-repo");
        assert_eq!(body["description"], "A test repo");
        assert_eq!(body["private"], true);
    }

    #[tokio::test]
    async fn get_pull_requests_sends_correct_request() {
        let response = r#"[
            {"id": 1, "number": 42, "title": "Fix bug", "state": "open", "created_at": "2026-01-01", "updated_at": "2026-01-02"}
        ]"#;
        let (base_url, request_rx) = start_github_server(response).await;

        let client = GitHubClient::with_base_url_for_test("ghp_test".to_string(), base_url);
        let prs = client
            .get_pull_requests("myorg", "myrepo", "open")
            .await
            .unwrap();

        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].number, 42);

        let request = request_rx.await.unwrap();
        assert!(request.starts_with(
            "GET /repos/myorg/myrepo/pulls?state=open HTTP/1.1"
        ));
    }

    #[tokio::test]
    async fn create_issue_sends_correct_payload() {
        let response = r#"{"id": 10, "number": 1, "title": "New issue", "body": "Details here", "state": "open", "labels": [{"name": "bug", "color": "ff0000"}], "created_at": "2026-01-01", "updated_at": "2026-01-01"}"#;
        let (base_url, request_rx) = start_github_server(response).await;

        let client = GitHubClient::with_base_url_for_test("ghp_test".to_string(), base_url);
        let issue = client
            .create_issue(
                "myorg",
                "myrepo",
                "New issue",
                "Details here",
                &["bug".to_string()],
            )
            .await
            .unwrap();

        assert_eq!(issue.title, "New issue");
        assert_eq!(issue.labels.len(), 1);
        assert_eq!(issue.labels[0].name, "bug");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /repos/myorg/myrepo/issues HTTP/1.1"));

        let body = request_body(&request);
        assert_eq!(body["title"], "New issue");
        assert_eq!(body["labels"][0], "bug");
    }

    #[tokio::test]
    async fn get_file_contents_decodes_base64() {
        use base64::Engine;
        let content = base64::engine::general_purpose::STANDARD
            .encode(b"fn main() {}");
        let response = format!(
            r#"{{"name": "main.rs", "path": "src/main.rs", "content": "{}", "encoding": "base64"}}"#,
            content
        );
        let (base_url, _request_rx) = start_github_server(Box::leak(response.into_boxed_str())).await;

        let client = GitHubClient::with_base_url_for_test("ghp_test".to_string(), base_url);
        let file_content = client
            .get_file_contents("myorg", "myrepo", "src/main.rs")
            .await
            .unwrap();

        assert_eq!(file_content, "fn main() {}");
    }

    #[tokio::test]
    async fn blank_token_rejected_before_network() {
        let client = GitHubClient::new("   ".to_string());
        let err = client
            .get_repositories("updated", 10)
            .await
            .unwrap_err();
        assert_eq!(err, "GitHub access token is required");
    }
}
