#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub content: String,
}
