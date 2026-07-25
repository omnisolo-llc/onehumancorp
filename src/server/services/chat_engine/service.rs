use std::sync::Arc;
use crate::db::DB;
use super::repo::{ChatEngineRepo, ChatMessage, ChatConversation};

pub struct ChatEngineService {
    repo: ChatEngineRepo,
}

impl ChatEngineService {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            repo: ChatEngineRepo::new(db.pool.clone()),
        }
    }

    pub async fn ingest_message(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        sender_type: &str,
        content: &str,
    ) -> Result<ChatMessage, String> {
        let mut msg = self.repo.create_message(tenant_id, conversation_id, sender_type, content)
            .await
            .map_err(|e| e.to_string())?;

        // POST_CREATE hook for AI drafting
        if sender_type != "agent" && sender_type != "system" {
            // Update status to pending
            let _ = self.repo.update_message_draft(tenant_id, &msg.id, "pending", None).await;

            let prompt = format!(
                "Analyze the following incoming customer message and provide a concise draft response. Message: {}",
                content
            );
            let prompt = crate::pricing::compression::reduce_tokens(&prompt);

            let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
                Ok("minimax") => {
                    if let Ok(api_key) = std::env::var("MINIMAX_API_KEY") {
                        crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
                    } else {
                        crate::minimax::LocalLLMClient::new().reason(&prompt).await
                    }
                }
                _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
            };

            if let Ok(draft_text) = llm_res {
                if let Ok(updated_msg) = self.repo.update_message_draft(tenant_id, &msg.id, "drafted", Some(&draft_text)).await {
                    msg = updated_msg;
                }
            } else {
                let _ = self.repo.update_message_draft(tenant_id, &msg.id, "failed", None).await;
            }
        }

        Ok(msg)
    }

    pub async fn get_conversations(&self, tenant_id: &str) -> Result<Vec<ChatConversation>, String> {
        self.repo.list_conversations(tenant_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_messages(&self, tenant_id: &str, conversation_id: &str) -> Result<Vec<ChatMessage>, String> {
        self.repo.list_messages(tenant_id, conversation_id).await.map_err(|e| e.to_string())
    }
}
