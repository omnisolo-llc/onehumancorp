#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
}
