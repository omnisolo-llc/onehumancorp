#[cfg(test)]
mod tests {
    use crate::domain::chat::models::Inbox;
    use uuid::Uuid;

    #[test]
    fn test_models_exist() {
        let tenant_id = Uuid::new_v4();
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id,
            name: "Test Inbox".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(inbox.name, "Test Inbox");
    }
}
