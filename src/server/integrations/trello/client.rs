use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrelloBoard {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrelloList {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub closed: Option<bool>,
    #[serde(default)]
    pub id_board: Option<String>,
    #[serde(default)]
    pub pos: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrelloCard {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub due: Option<String>,
    #[serde(default)]
    pub id_list: Option<String>,
    #[serde(default)]
    pub id_board: Option<String>,
    #[serde(default)]
    pub closed: Option<bool>,
    #[serde(default)]
    pub short_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrelloLabel {
    pub id: String,
    pub name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

pub struct TrelloClient {
    pub api_key: String,
    pub token: String,
    http_client: Client,
    base_url: String,
}

impl TrelloClient {
    pub fn new(api_key: String, token: String) -> Self {
        Self {
            api_key,
            token,
            http_client: Client::new(),
            base_url: "https://api.trello.com/1".to_string(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(api_key: String, token: String, base_url: String) -> Self {
        Self {
            api_key,
            token,
            http_client: Client::new(),
            base_url,
        }
    }

    fn api_url(&self, path: &str) -> String {
        format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            path
        )
    }

    fn validate_credentials(&self) -> Result<(), String> {
        let key = self.api_key.trim();
        let token = self.token.trim();
        if key.is_empty() || token.is_empty() {
            return Err("Trello API key and token are required".to_string());
        }
        Ok(())
    }

    pub async fn get_boards(&self) -> Result<Vec<TrelloBoard>, String> {
        self.validate_credentials()?;
        let url = self.api_url(&format!(
            "/members/me/boards?key={}&token={}",
            self.api_key, self.token
        ));
        let resp = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error fetching boards: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error fetching boards: {status} - {body}"));
        }
        resp.json::<Vec<TrelloBoard>>()
            .await
            .map_err(|e| format!("Failed to parse boards response: {e}"))
    }

    pub async fn get_lists(&self, board_id: &str) -> Result<Vec<TrelloList>, String> {
        self.validate_credentials()?;
        let url = self.api_url(&format!(
            "/boards/{}/lists?key={}&token={}",
            board_id, self.api_key, self.token
        ));
        let resp = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error fetching lists: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error fetching lists: {status} - {body}"));
        }
        resp.json::<Vec<TrelloList>>()
            .await
            .map_err(|e| format!("Failed to parse lists response: {e}"))
    }

    pub async fn get_cards(&self, list_id: &str) -> Result<Vec<TrelloCard>, String> {
        self.validate_credentials()?;
        let url = self.api_url(&format!(
            "/lists/{}/cards?key={}&token={}",
            list_id, self.api_key, self.token
        ));
        let resp = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error fetching cards: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error fetching cards: {status} - {body}"));
        }
        resp.json::<Vec<TrelloCard>>()
            .await
            .map_err(|e| format!("Failed to parse cards response: {e}"))
    }

    pub async fn create_card(
        &self,
        list_id: &str,
        name: &str,
        description: Option<&str>,
        due_date: Option<&str>,
        label_ids: &[String],
    ) -> Result<TrelloCard, String> {
        self.validate_credentials()?;
        let mut payload = serde_json::json!({
            "idList": list_id,
            "name": name,
            "key": self.api_key,
            "token": self.token,
        });
        if let Some(desc) = description {
            payload["desc"] = serde_json::json!(desc);
        }
        if let Some(due) = due_date {
            payload["due"] = serde_json::json!(due);
        }
        if !label_ids.is_empty() {
            payload["idLabels"] = serde_json::json!(label_ids.join(","));
        }

        let url = self.api_url("/cards");
        let resp = self
            .http_client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating card: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error creating card: {status} - {body}"));
        }
        resp.json::<TrelloCard>()
            .await
            .map_err(|e| format!("Failed to parse create card response: {e}"))
    }

    pub async fn update_card(
        &self,
        card_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        due_date: Option<&str>,
    ) -> Result<TrelloCard, String> {
        self.validate_credentials()?;
        let url = self.api_url(&format!(
            "/cards/{}?key={}&token={}",
            card_id, self.api_key, self.token
        ));
        let mut payload = serde_json::json!({});
        if let Some(n) = name {
            payload["name"] = serde_json::json!(n);
        }
        if let Some(desc) = description {
            payload["desc"] = serde_json::json!(desc);
        }
        if let Some(due) = due_date {
            payload["due"] = serde_json::json!(due);
        }

        let resp = self
            .http_client
            .put(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error updating card: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error updating card: {status} - {body}"));
        }
        resp.json::<TrelloCard>()
            .await
            .map_err(|e| format!("Failed to parse update card response: {e}"))
    }

    pub async fn move_card(
        &self,
        card_id: &str,
        list_id: &str,
    ) -> Result<TrelloCard, String> {
        self.validate_credentials()?;
        let url = self.api_url(&format!(
            "/cards/{}?idList={}&key={}&token={}",
            card_id, list_id, self.api_key, self.token
        ));

        let resp = self
            .http_client
            .put(&url)
            .send()
            .await
            .map_err(|e| format!("Network error moving card: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error moving card: {status} - {body}"));
        }
        resp.json::<TrelloCard>()
            .await
            .map_err(|e| format!("Failed to parse move card response: {e}"))
    }

    pub async fn delete_card(&self, card_id: &str) -> Result<(), String> {
        self.validate_credentials()?;
        let url = self.api_url(&format!(
            "/cards/{}?key={}&token={}",
            card_id, self.api_key, self.token
        ));

        let resp = self
            .http_client
            .delete(&url)
            .send()
            .await
            .map_err(|e| format!("Network error deleting card: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error deleting card: {status} - {body}"));
        }
        Ok(())
    }

    pub async fn get_labels(&self, board_id: &str) -> Result<Vec<TrelloLabel>, String> {
        self.validate_credentials()?;
        let url = self.api_url(&format!(
            "/boards/{}/labels?key={}&token={}",
            board_id, self.api_key, self.token
        ));
        let resp = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error fetching labels: {e}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Trello API error fetching labels: {status} - {body}"));
        }
        resp.json::<Vec<TrelloLabel>>()
            .await
            .map_err(|e| format!("Failed to parse labels response: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_mock_server(
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

    fn request_path(request: &str) -> &str {
        request.split_whitespace().nth(1).unwrap_or("")
    }

    fn request_method(request: &str) -> &str {
        request.split_whitespace().next().unwrap_or("")
    }

    #[tokio::test]
    async fn get_boards_returns_boards() {
        let response = r#"[
            {"id": "board1", "name": "My Board", "desc": "Test board", "url": "https://trello.com/b/board1"},
            {"id": "board2", "name": "Another Board"}
        ]"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let boards = client.get_boards().await.unwrap();
        assert_eq!(boards.len(), 2);
        assert_eq!(boards[0].id, "board1");
        assert_eq!(boards[0].name, "My Board");
        assert_eq!(boards[1].name, "Another Board");

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "GET");
        assert!(request_path(&request).contains("/members/me/boards"));
    }

    #[tokio::test]
    async fn get_lists_returns_lists() {
        let response = r#"[
            {"id": "list1", "name": "To Do", "closed": false, "idBoard": "board1", "pos": 1.0},
            {"id": "list2", "name": "Done", "closed": false, "idBoard": "board1", "pos": 2.0}
        ]"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let lists = client.get_lists("board1").await.unwrap();
        assert_eq!(lists.len(), 2);
        assert_eq!(lists[0].id, "list1");
        assert_eq!(lists[0].name, "To Do");

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "GET");
        assert!(request_path(&request).contains("/boards/board1/lists"));
    }

    #[tokio::test]
    async fn get_cards_returns_cards() {
        let response = r#"[
            {"id": "card1", "name": "Task 1", "desc": "Do something", "due": "2026-08-01", "idList": "list1"}
        ]"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let cards = client.get_cards("list1").await.unwrap();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].id, "card1");
        assert_eq!(cards[0].name, "Task 1");

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "GET");
        assert!(request_path(&request).contains("/lists/list1/cards"));
    }

    #[tokio::test]
    async fn create_card_returns_card() {
        let response = r#"{
            "id": "card_new",
            "name": "New Task",
            "desc": "A description",
            "due": "2026-09-01",
            "idList": "list1"
        }"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let card = client
            .create_card(
                "list1",
                "New Task",
                Some("A description"),
                Some("2026-09-01"),
                &[],
            )
            .await
            .unwrap();
        assert_eq!(card.id, "card_new");
        assert_eq!(card.name, "New Task");

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "POST");
        assert!(request_path(&request).contains("/cards"));
    }

    #[tokio::test]
    async fn update_card_returns_card() {
        let response = r#"{
            "id": "card1",
            "name": "Updated Task",
            "desc": "Updated desc",
            "due": "2026-10-01",
            "idList": "list1"
        }"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let card = client
            .update_card(
                "card1",
                Some("Updated Task"),
                Some("Updated desc"),
                Some("2026-10-01"),
            )
            .await
            .unwrap();
        assert_eq!(card.id, "card1");
        assert_eq!(card.name, "Updated Task");

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "PUT");
        assert!(request_path(&request).contains("/cards/card1"));
    }

    #[tokio::test]
    async fn move_card_returns_card() {
        let response = r#"{
            "id": "card1",
            "name": "Task",
            "idList": "list2"
        }"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let card = client.move_card("card1", "list2").await.unwrap();
        assert_eq!(card.id, "card1");
        assert_eq!(card.id_list.as_deref(), Some("list2"));

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "PUT");
        assert!(request_path(&request).contains("/cards/card1"));
        assert!(request.contains("idList=list2"));
    }

    #[tokio::test]
    async fn delete_card_returns_ok() {
        let response = r#"{"_value": null}"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        client.delete_card("card1").await.unwrap();

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "DELETE");
        assert!(request_path(&request).contains("/cards/card1"));
    }

    #[tokio::test]
    async fn get_labels_returns_labels() {
        let response = r#"[
            {"id": "label1", "name": "Urgent", "color": "red"},
            {"id": "label2", "name": "Low", "color": "green"}
        ]"#;

        let (base_url, request_rx) = start_mock_server(response).await;
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let labels = client.get_labels("board1").await.unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].name.as_deref(), Some("Urgent"));

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "GET");
        assert!(request_path(&request).contains("/boards/board1/labels"));
    }

    #[tokio::test]
    async fn credentials_validation_rejects_empty_key() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let client = TrelloClient::with_base_url_for_test(
            "".to_string(),
            "test-token".to_string(),
            base_url,
        );

        let err = client.get_boards().await.unwrap_err();
        assert!(err.contains("Trello API key and token are required"));
    }

    #[tokio::test]
    async fn credentials_validation_rejects_empty_token() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let client = TrelloClient::with_base_url_for_test(
            "test-key".to_string(),
            "".to_string(),
            base_url,
        );

        let err = client.get_boards().await.unwrap_err();
        assert!(err.contains("Trello API key and token are required"));
    }

    #[tokio::test]
    async fn client_new_sets_fields() {
        let client = TrelloClient::new("key123".to_string(), "tok456".to_string());
        assert_eq!(client.api_key, "key123");
        assert_eq!(client.token, "tok456");
        assert_eq!(client.base_url, "https://api.trello.com/1");
    }
}
