// Temporarily using String instead of uuid::Uuid if uuid isn't easily available,
// or we can just use simple types to make it compile for now without extra deps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
}
