#[cfg(test)]
mod tests {
    use super::super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_message() {
        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let msg = create_message(tenant_id, conversation_id, "hello".to_string()).await.unwrap();
        assert_eq!(msg.payload, "hello");
        assert_eq!(msg.tenant_id, tenant_id);
    }
}
