use super::models::Conversation;


pub struct Router {
    pub agents_online: Vec<String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            agents_online: Vec::new(),
        }
    }

    pub fn assign_conversation(&self, conv: &mut Conversation) -> Result<(), String> {
        // Implement simple round-robin or skill-based routing
        if self.agents_online.is_empty() {
            // Assign to bot or keep unassigned
            conv.assignee_id = None;
            return Ok(());
        }

        // Just pick the first online agent for now
        conv.assignee_id = Some(self.agents_online[0].clone());
        Ok(())
    }
}
