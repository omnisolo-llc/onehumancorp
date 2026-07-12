use std::sync::Arc;

/// Immutable authority identifying the only tenant a built-in agent process
/// may access.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantContext(Arc<str>);

impl TenantContext {
    pub fn new(organization_id: impl AsRef<str>) -> Result<Self, &'static str> {
        let organization_id = organization_id.as_ref().trim();
        if organization_id.is_empty() {
            return Err("organization ID must not be empty");
        }

        Ok(Self(Arc::from(organization_id)))
    }

    pub fn system() -> Self {
        Self(Arc::from("system"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
