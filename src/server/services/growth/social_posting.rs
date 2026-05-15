use std::sync::RwLock;

#[derive(Clone, Debug)]
pub struct SocialPost {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub content: String,
    pub status: String,
}

pub struct SocialPostingAgent {
    posts: RwLock<Vec<SocialPost>>,
}

impl SocialPostingAgent {
    pub fn new() -> Self {
        SocialPostingAgent {
            posts: RwLock::new(Vec::new()),
        }
    }

    pub fn generate_post(&self, user_id: &str, platform: &str, topic: &str) -> SocialPost {
        let content = match topic {
            "new_product" => format!("Check out our latest arrival on {}! 🚀", platform),
            "sale" => format!("Huge flash sale today! Don't miss out. Link in bio."),
            _ => format!("Exciting news coming soon on {}!", platform),
        };

        let post = SocialPost {
            id: format!("post-{}-{}", user_id, chrono::Utc::now().timestamp()),
            user_id: user_id.to_string(),
            platform: platform.to_string(),
            content,
            status: "PENDING_APPROVAL".to_string(),
        };

        let mut list = self.posts.write().unwrap();
        list.push(post.clone());
        post
    }

    pub fn approve_post(&self, post_id: &str) -> Result<SocialPost, String> {
        let mut list = self.posts.write().unwrap();
        if let Some(p) = list.iter_mut().find(|p| p.id == post_id) {
            p.status = "APPROVED".to_string();
            return Ok(p.clone());
        }
        Err("Post not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_posting_flow() {
        let agent = SocialPostingAgent::new();
        let post = agent.generate_post("user1", "Instagram", "new_product");
        assert_eq!(post.status, "PENDING_APPROVAL");

        let approved = agent.approve_post(&post.id).unwrap();
        assert_eq!(approved.status, "APPROVED");

        let err = agent.approve_post("nonexistent").unwrap_err();
        assert_eq!(err, "Post not found");
    }
}
