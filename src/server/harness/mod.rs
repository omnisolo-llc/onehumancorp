pub mod sandbox;
pub mod executor;
pub mod bwrap_executor;
pub mod network_proxy;

pub use bwrap_executor::BwrapExecutor;
pub use network_proxy::NetworkProxy;

pub async fn start_sandbox_proxy(pool: Option<sqlx::PgPool>, allowed_domains: Vec<String>, port: u16) -> Result<(), String> {
    let proxy = NetworkProxy::new(pool, allowed_domains);
    tokio::spawn(async move {
        let _ = proxy.start(port).await;
    });
    Ok(())
}
