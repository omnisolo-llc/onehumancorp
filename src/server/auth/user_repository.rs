use async_trait::async_trait;
use super::User;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User, org_id: &str) -> Result<(), String>;
    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String>;
    async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String>;
    async fn get_by_email(&self, email: &str, org_id: &str) -> Result<User, String>;
    async fn get_by_login_identifier(
        &self,
        identifier: &str,
        org_id: &str,
    ) -> Result<Option<User>, String>;
    async fn get_by_oidc_subject(&self, sub: &str, org_id: &str) -> Result<User, String>;
    async fn list_users(&self, org_id: &str) -> Result<Vec<User>, String>;
    async fn update_user(&self, user: User, org_id: &str) -> Result<(), String>;
    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String>;
    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, org_id: &str) -> Result<(), String>;
    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String>;
}
