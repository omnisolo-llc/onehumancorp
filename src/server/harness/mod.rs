pub mod executor;
pub mod telemetry;
pub mod sandbox;
pub mod mcp;

pub mod network_proxy;


#[derive(serde::Deserialize)]
pub struct UiTenantQuery {
    pub tenant_id: Option<String>,
    pub tenant: Option<String>,
    pub mobile_optimized: Option<bool>,
}

pub fn ui_tenant_id(query: &UiTenantQuery) -> String {
    query
        .tenant_id
        .as_deref()
        .or(query.tenant.as_deref())
        .map(str::trim)
        .filter(|tenant| !tenant.is_empty())
        .unwrap_or("default")
        .to_string()
}
