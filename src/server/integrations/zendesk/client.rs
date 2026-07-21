use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZendeskTicket {
    pub id: Option<u64>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub requester_id: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZendeskComment {
    pub id: Option<u64>,
    pub body: Option<String>,
    pub author_id: Option<u64>,
    pub created_at: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZendeskUser {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct TicketResponse {
    ticket: ZendeskTicket,
}

#[derive(Debug, Deserialize)]
struct TicketsResponse {
    tickets: Vec<ZendeskTicket>,
}

#[derive(Debug, Deserialize)]
struct CommentsResponse {
    comments: Vec<ZendeskComment>,
}

#[derive(Debug, Deserialize)]
struct UserResponse {
    user: ZendeskUser,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<ZendeskTicket>,
}

pub struct ZendeskClient {
    pub base_url: String,
    pub email: String,
    pub api_token: String,
    http_client: Client,
}

impl ZendeskClient {
    pub fn new(subdomain: String, email: String, api_token: String) -> Self {
        let base_url = format!("https://{}.zendesk.com", subdomain);
        Self {
            base_url,
            email,
            api_token,
            http_client: Client::new(),
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> Result<reqwest::Response, String> {
        let url = format!("{}{}", self.base_url, path);
        let auth_format = format!("{}/token:{}", self.email, self.api_token);

        let req = match method {
            reqwest::Method::GET => self.http_client.get(&url),
            reqwest::Method::POST => self.http_client.post(&url),
            reqwest::Method::PUT => self.http_client.put(&url),
            _ => return Err(format!("unsupported method: {}", method)),
        };

        let res = req
            .basic_auth(&auth_format, None::<&str>)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Zendesk API HTTP {}: {}", status, body));
        }
        Ok(res)
    }

    pub async fn create_ticket(
        &self,
        subject: &str,
        description: &str,
        priority: &str,
        requester_email: &str,
    ) -> Result<ZendeskTicket, String> {
        let payload = serde_json::json!({
            "ticket": {
                "subject": subject,
                "description": description,
                "priority": priority,
                "requester": {"name": "", "email": requester_email}
            }
        });

        let url = format!("{}/api/v2/tickets.json", self.base_url);
        let auth_format = format!("{}/token:{}", self.email, self.api_token);

        let res = self
            .http_client
            .post(&url)
            .basic_auth(&auth_format, None::<&str>)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Zendesk API HTTP {}: {}", status, body));
        }

        let resp: TicketResponse = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(resp.ticket)
    }

    pub async fn get_ticket(&self, ticket_id: u64) -> Result<ZendeskTicket, String> {
        let path = format!("/api/v2/tickets/{}.json", ticket_id);
        let res = self.request(reqwest::Method::GET, &path).await?;
        let resp: TicketResponse = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(resp.ticket)
    }

    pub async fn update_ticket(
        &self,
        ticket_id: u64,
        comment: Option<&str>,
        status: Option<&str>,
        priority: Option<&str>,
    ) -> Result<ZendeskTicket, String> {
        let mut ticket = serde_json::json!({});

        if let Some(c) = comment {
            ticket["comment"] = serde_json::json!({"body": c});
        }
        if let Some(s) = status {
            ticket["status"] = serde_json::json!(s);
        }
        if let Some(p) = priority {
            ticket["priority"] = serde_json::json!(p);
        }

        let payload = serde_json::json!({ "ticket": ticket });
        let path = format!("/api/v2/tickets/{}.json", ticket_id);
        let url = format!("{}{}", self.base_url, path);
        let auth_format = format!("{}/token:{}", self.email, self.api_token);

        let res = self
            .http_client
            .put(&url)
            .basic_auth(&auth_format, None::<&str>)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status_code = res.status();
        if !status_code.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Zendesk API HTTP {}: {}", status_code, body));
        }

        let resp: TicketResponse = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(resp.ticket)
    }

    pub async fn list_tickets(
        &self,
        status: Option<&str>,
        limit: u32,
    ) -> Result<Vec<ZendeskTicket>, String> {
        let mut path = format!("/api/v2/tickets.json?per_page={}", limit);
        if let Some(s) = status {
            path.push_str(&format!("&status={}", s));
        }
        let res = self.request(reqwest::Method::GET, &path).await?;
        let resp: TicketsResponse = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(resp.tickets)
    }

    pub async fn search_tickets(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<ZendeskTicket>, String> {
        let path = format!(
            "/api/v2/search.json?query={}&per_page={}",
            urlencoding::encode(query),
            limit
        );
        let res = self.request(reqwest::Method::GET, &path).await?;
        let resp: SearchResponse = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(resp.results)
    }

    pub async fn get_ticket_comments(
        &self,
        ticket_id: u64,
    ) -> Result<Vec<ZendeskComment>, String> {
        let path = format!("/api/v2/tickets/{}/comments.json", ticket_id);
        let res = self.request(reqwest::Method::GET, &path).await?;
        let resp: CommentsResponse = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(resp.comments)
    }

    pub async fn add_comment(
        &self,
        ticket_id: u64,
        body: &str,
        author_id: u64,
    ) -> Result<(), String> {
        let payload = serde_json::json!({
            "ticket": {
                "comment": {
                    "body": body,
                    "author_id": author_id
                }
            }
        });

        let path = format!("/api/v2/tickets/{}.json", ticket_id);
        let url = format!("{}{}", self.base_url, path);
        let auth_format = format!("{}/token:{}", self.email, self.api_token);

        let res = self
            .http_client
            .put(&url)
            .basic_auth(&auth_format, None::<&str>)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Zendesk API HTTP {}: {}", status, body));
        }
        Ok(())
    }

    pub async fn create_user(
        &self,
        name: &str,
        email: &str,
        role: &str,
    ) -> Result<ZendeskUser, String> {
        let payload = serde_json::json!({
            "user": {
                "name": name,
                "email": email,
                "role": role
            }
        });

        let url = format!("{}/api/v2/users.json", self.base_url);
        let auth_format = format!("{}/token:{}", self.email, self.api_token);

        let res = self
            .http_client
            .post(&url)
            .basic_auth(&auth_format, None::<&str>)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Zendesk API HTTP {}: {}", status, body));
        }

        let resp: UserResponse = res
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {}", e))?;
        Ok(resp.user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;

    fn start_mock_server(responses: Vec<String>) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            for resp_body in responses {
                if let Ok(stream) = listener.accept() {
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
                    let _ = std::io::Read::read_exact(&mut stream, &mut body);

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        resp_body.len(),
                        resp_body
                    );
                    let _ = stream.write_all(response.as_bytes());
                }
            }
        });

        port
    }

    #[tokio::test]
    async fn test_create_ticket() {
        let resp = serde_json::json!({
            "ticket": {
                "id": 12345,
                "subject": "Test ticket",
                "description": "Test description",
                "status": "open",
                "priority": "normal"
            }
        })
        .to_string();

        let _port = start_mock_server(vec![resp.clone()]);
        let resp: TicketResponse = serde_json::from_str(&resp).unwrap();
        assert_eq!(resp.ticket.id, Some(12345));
        assert_eq!(resp.ticket.subject.as_deref(), Some("Test ticket"));
        assert_eq!(resp.ticket.status.as_deref(), Some("open"));
    }

    #[tokio::test]
    async fn test_get_ticket() {
        let resp = serde_json::json!({
            "ticket": {
                "id": 99,
                "subject": "Help me",
                "status": "pending",
                "priority": "high"
            }
        })
        .to_string();

        let resp: TicketResponse = serde_json::from_str(&resp).unwrap();
        assert_eq!(resp.ticket.id, Some(99));
        assert_eq!(resp.ticket.priority.as_deref(), Some("high"));
    }

    #[tokio::test]
    async fn test_list_tickets() {
        let resp = serde_json::json!({
            "tickets": [
                {"id": 1, "subject": "A", "status": "new"},
                {"id": 2, "subject": "B", "status": "open"}
            ]
        })
        .to_string();

        let resp: TicketsResponse = serde_json::from_str(&resp).unwrap();
        assert_eq!(resp.tickets.len(), 2);
        assert_eq!(resp.tickets[0].id, Some(1));
    }

    #[tokio::test]
    async fn test_search_tickets() {
        let resp = serde_json::json!({
            "results": [
                {"id": 10, "subject": "Bug report", "status": "solved"}
            ]
        })
        .to_string();

        let resp: SearchResponse = serde_json::from_str(&resp).unwrap();
        assert_eq!(resp.results.len(), 1);
        assert_eq!(resp.results[0].subject.as_deref(), Some("Bug report"));
    }

    #[tokio::test]
    async fn test_get_ticket_comments() {
        let resp = serde_json::json!({
            "comments": [
                {"id": 100, "body": "First comment", "author_id": 1},
                {"id": 101, "body": "Second comment", "author_id": 2}
            ]
        })
        .to_string();

        let resp: CommentsResponse = serde_json::from_str(&resp).unwrap();
        assert_eq!(resp.comments.len(), 2);
        assert_eq!(resp.comments[0].body.as_deref(), Some("First comment"));
    }

    #[tokio::test]
    async fn test_create_user() {
        let resp = serde_json::json!({
            "user": {
                "id": 500,
                "name": "Jane Doe",
                "email": "jane@example.com",
                "role": "agent"
            }
        })
        .to_string();

        let resp: UserResponse = serde_json::from_str(&resp).unwrap();
        assert_eq!(resp.user.id, Some(500));
        assert_eq!(resp.user.role.as_deref(), Some("agent"));
    }

    #[test]
    fn test_client_new() {
        let client =
            ZendeskClient::new("mycompany".into(), "admin@my.com".into(), "tok123".into());
        assert_eq!(client.base_url, "https://mycompany.zendesk.com");
        assert_eq!(client.email, "admin@my.com");
    }

    #[test]
    fn test_ticket_deserialize() {
        let json = serde_json::json!({
            "id": 42,
            "subject": "Login issue",
            "status": "open",
            "priority": "urgent",
            "created_at": "2025-01-01T00:00:00Z"
        });
        let ticket: ZendeskTicket = serde_json::from_value(json).unwrap();
        assert_eq!(ticket.id, Some(42));
        assert_eq!(ticket.created_at.as_deref(), Some("2025-01-01T00:00:00Z"));
    }

    #[tokio::test]
    async fn test_mock_server_e2e() {
        let resp = serde_json::json!({
            "ticket": {
                "id": 777,
                "subject": "E2E test",
                "status": "new",
                "priority": "low"
            }
        })
        .to_string();

        let port = start_mock_server(vec![resp]);
        let client = ZendeskClient {
            base_url: format!("http://127.0.0.1:{}", port),
            email: "test@example.com".into(),
            api_token: "token123".into(),
            http_client: Client::new(),
        };

        let ticket = client.get_ticket(777).await.unwrap();
        assert_eq!(ticket.id, Some(777));
        assert_eq!(ticket.subject.as_deref(), Some("E2E test"));
    }
}
