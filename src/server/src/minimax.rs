use serde::{Serialize, Deserialize};

pub struct MinimaxClient {
    api_key: String,
    url: String,
}

#[derive(Debug, Serialize)]
struct MinimaxRequest {
    model: String,
    messages: Vec<MinimaxMessage>,
}

#[derive(Debug, Serialize)]
struct MinimaxMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct MinimaxResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    content: String,
}

impl MinimaxClient {
    pub fn new(api_key: String) -> Self {
        MinimaxClient {
            api_key,
            url: "https://api.minimax.chat/v1/chat/completions".to_string(),
        }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        
        let request_body = MinimaxRequest {
            model: "MiniMax-M2.7".to_string(),
            messages: vec![MinimaxMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };
        
        let response = client.post(&self.url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
            
        if response.status().is_success() {
            let result: MinimaxResponse = response.json().await.map_err(|e| e.to_string())?;
            if let Some(choice) = result.choices.first() {
                Ok(choice.message.content.clone())
            } else {
                Err("empty response from minimax".to_string())
            }
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(format!("API error (status {}): {}", status, text))
        }
    }
}
